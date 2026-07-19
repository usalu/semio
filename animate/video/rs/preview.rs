use animate_core::{preview_scene_loop, AnimateConfig, Scene, SceneFrame};
use std::io::Write;

/// 🪟 Live preview outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewOutcome {
    FrameLimit,
    WindowClosed,
    MetadataOnly,
}

/// 🖥️ Previews a scene in a wgpu window when `preview-window` is enabled, else logs frame metadata.
pub fn preview_scene_window<S: Scene>(
    mut scene: S,
    config: &AnimateConfig,
    max_frames: Option<u64>,
) -> Result<PreviewOutcome, String> {
    scene.setup(config);
    #[cfg(feature = "preview-window")]
    {
        return preview_scene_window_winit(scene, config, max_frames);
    }
    #[cfg(not(feature = "preview-window"))]
    {
        let outcome = preview_scene_window_metadata(&mut scene, max_frames)?;
        scene.tear_down();
        Ok(outcome)
    }
}

#[cfg(feature = "preview-window")]
fn preview_scene_window_winit<S: Scene>(
    mut scene: S,
    config: &AnimateConfig,
    max_frames: Option<u64>,
) -> Result<PreviewOutcome, String> {
    use crate::renderer::{CapturedFrame, VelloRenderer};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use winit::application::ApplicationHandler;
    use winit::event::WindowEvent;
    use winit::event_loop::{ActiveEventLoop, EventLoop};
    use winit::window::{Window, WindowId};

    struct PreviewApp<S> {
        scene: S,
        config: AnimateConfig,
        max_frames: u64,
        frame_index: u64,
        renderer: Option<VelloRenderer>,
        window: Option<Arc<Window>>,
        closed: Arc<AtomicBool>,
        constructed: bool,
        error: Option<String>,
    }

    impl<S: Scene> PreviewApp<S> {
        fn fail(&mut self, message: String) {
            self.error = Some(message);
        }
    }

    impl<S: Scene> ApplicationHandler for PreviewApp<S> {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            if self.window.is_some() {
                return;
            }
            let window = match event_loop.create_window(
                Window::default_attributes()
                    .with_title("Animate Preview")
                    .with_inner_size(winit::dpi::LogicalSize::new(self.config.width, self.config.height)),
            ) {
                Ok(window) => Arc::new(window),
                Err(err) => {
                    self.fail(format!("preview window: {err}"));
                    event_loop.exit();
                    return;
                }
            };
            match VelloRenderer::new(self.config.width, self.config.height) {
                Ok(renderer) => self.renderer = Some(renderer),
                Err(err) => {
                    self.fail(err);
                    event_loop.exit();
                    return;
                }
            }
            self.window = Some(window.clone());
            if !self.constructed {
                self.scene.construct();
                self.constructed = true;
            }
            window.request_redraw();
        }

        fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
            match event {
                WindowEvent::CloseRequested => {
                    self.closed.store(true, Ordering::Relaxed);
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    if self.frame_index >= self.max_frames {
                        event_loop.exit();
                        return;
                    }
                    self.scene.sample_frame(self.config.frame_duration());
                    let capture = CapturedFrame {
                        time: self.scene.scene_time(),
                        mobjects: self.scene.mobjects().values().map(|m| m.clone_box()).collect(),
                    };
                    if let Some(renderer) = self.renderer.as_mut() {
                        let _ = renderer.render_capture(&capture, self.scene.camera(), &self.config);
                    }
                    let frame = self.scene.render_frame_index(self.frame_index);
                    if let Some(window) = self.window.as_ref() {
                        window.set_title(&format!(
                            "Animate Preview — frame {} t={:.2}s mobjects={} section={:?}",
                            frame.frame, frame.time, frame.mobject_count, frame.section
                        ));
                        if self.frame_index + 1 < self.max_frames {
                            window.request_redraw();
                        } else {
                            event_loop.exit();
                        }
                    }
                    self.frame_index += 1;
                }
                _ => {}
            }
        }
    }

    let max = max_frames.unwrap_or(300);
    let mut app = PreviewApp {
        scene,
        config: config.clone(),
        max_frames: max,
        frame_index: 0,
        renderer: None,
        window: None,
        closed: Arc::new(AtomicBool::new(false)),
        constructed: false,
        error: None,
    };
    let event_loop = EventLoop::new().map_err(|err| format!("preview event loop: {err}"))?;
    event_loop
        .run_app(&mut app)
        .map_err(|err| format!("preview run: {err}"))?;
    app.scene.tear_down();
    if let Some(error) = app.error {
        return Err(error);
    }
    if app.closed.load(Ordering::Relaxed) {
        Ok(PreviewOutcome::WindowClosed)
    } else if app.frame_index >= max {
        Ok(PreviewOutcome::FrameLimit)
    } else {
        Ok(PreviewOutcome::WindowClosed)
    }
}

fn preview_scene_window_metadata<S: Scene>(scene: &mut S, max_frames: Option<u64>) -> Result<PreviewOutcome, String> {
    let max = max_frames.unwrap_or(120);
    let mut stderr = std::io::stderr();
    preview_scene_loop(scene, max, |frame: &SceneFrame| {
        let _ = writeln!(
            stderr,
            "[animate-preview] frame={} time={:.3}s mobjects={} section={:?}",
            frame.frame, frame.time, frame.mobject_count, frame.section
        );
    });
    Ok(PreviewOutcome::MetadataOnly)
}

/// 🧪 Headless preview used by CLI `--preview` flag.
pub fn preview_scene_headless<S: Scene>(scene: S, config: &AnimateConfig, max_frames: Option<u64>) -> Result<PreviewOutcome, String> {
    preview_scene_window(scene, config, max_frames)
}

#[cfg(test)]
mod tests {
    use super::*;
    use animate_core::{BasicScene, Camera, Scene, SectionList, Sobject, VSobject};
    use std::collections::HashMap;

    struct DemoScene {
        base: BasicScene,
    }

    impl DemoScene {
        fn new(config: AnimateConfig) -> Self {
            Self {
                base: BasicScene::new(config),
            }
        }
    }

    impl Scene for DemoScene {
        fn construct(&mut self) {
            self.add(Box::new(VSobject::new()));
            self.wait(0.05);
        }
        fn config(&self) -> &AnimateConfig {
            self.base.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.base.config_mut()
        }
        fn camera(&self) -> &Camera {
            self.base.camera()
        }
        fn camera_mut(&mut self) -> &mut Camera {
            self.base.camera_mut()
        }
        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
            self.base.mobjects()
        }
        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
            self.base.mobjects_mut()
        }
        fn sections(&self) -> &SectionList {
            self.base.sections()
        }
        fn sections_mut(&mut self) -> &mut SectionList {
            self.base.sections_mut()
        }
        fn scene_time(&self) -> f64 {
            self.base.scene_time()
        }
        fn set_scene_time(&mut self, time: f64) {
            self.base.set_scene_time(time);
        }
    }

    #[test]
    fn preview_scene_window_metadata_runs() {
        let config = AnimateConfig::default().with_resolution(64, 64).with_frame_rate(30.0);
        let scene = DemoScene::new(config.clone());
        let outcome = preview_scene_headless(scene, &config, Some(2)).expect("preview");
        assert_eq!(outcome, PreviewOutcome::MetadataOnly);
    }
}
