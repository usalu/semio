//! 🎞️ Exercise the production file writer with a small fixture and the native codec archives.

mod editor {
    pub mod animate {
        pub mod engine {
            pub mod config {
                pub mod config {
                    use std::path::{Path, PathBuf};

                    #[derive(Clone, Default)]
                    pub struct Cache {
                        pub partial_movie_dir: PathBuf,
                    }

                    #[derive(Clone)]
                    pub struct AnimateConfig {
                        pub width: u32,
                        pub height: u32,
                        pub background: [f64; 4],
                        pub frame_rate: f64,
                        pub output_dir: PathBuf,
                        pub media_dir: PathBuf,
                        pub cache: Cache,
                    }

                    impl Default for AnimateConfig {
                        fn default() -> Self {
                            Self { width: 16, height: 16, frame_rate: 24.0, background: [0.0, 0.0, 0.0, 1.0], output_dir: PathBuf::new(), media_dir: PathBuf::new(), cache: Cache::default() }
                        }
                    }

                    impl AnimateConfig {
                        pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
                            (self.width, self.height) = (width, height);
                            self
                        }

                        pub fn with_output_dir(mut self, path: impl AsRef<Path>) -> Self {
                            self.output_dir = path.as_ref().into();
                            self
                        }

                        pub fn with_media_dir(mut self, path: impl AsRef<Path>) -> Self {
                            self.media_dir = path.as_ref().into();
                            self
                        }
                    }
                }
            }

            pub mod scene {
                pub mod section {
                    #[derive(Default)]
                    pub struct SectionList {
                        pub sections: Vec<Section>,
                    }

                    pub struct Section {
                        pub name: String,
                        pub start_time: f64,
                        pub end_time: f64,
                    }
                }
            }

            pub mod video {
                pub mod render {
                    use std::path::PathBuf;

                    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                    pub enum OutputFormat {
                        Mp4,
                        Gif,
                        PngSequence,
                        LastFrame,
                    }

                    #[derive(Default)]
                    pub struct OutputPaths {
                        pub mp4: Option<PathBuf>,
                        pub gif: Option<PathBuf>,
                        pub png_dir: Option<PathBuf>,
                        pub last_frame: Option<PathBuf>,
                    }
                }

                #[path = "SEMIO_NATIVE_ANIMATE_CACHE"]
                pub mod cache;

                #[path = "SEMIO_NATIVE_ANIMATE_WRITER"]
                pub mod writer;

                #[derive(Debug)]
                pub enum VideoError {
                    Report(String),
                    CacheEvictionEmpty,
                }

                impl VideoError {
                    pub fn backend(context: &'static str, error: impl std::fmt::Display) -> Self {
                        Self::Report(format!("{context}: {error}"))
                    }

                    pub fn io(context: &'static str) -> impl Fn(std::io::Error) -> Self {
                        move |error| Self::backend(context, error)
                    }
                }
            }
        }
    }
}

use editor::animate::engine::config::config::AnimateConfig;
use editor::animate::engine::video::{render::OutputFormat, writer::SceneFileWriter};
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::io::encode_mp4;
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Sample, Mp4Snapshot, Mp4Track};
use std::{fs, path::PathBuf};

fn main() {
    let out = PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    let dark = [6, 26, 31, 255];
    let light = [233, 230, 215, 255];
    let dark_chip = [17, 35, 40, 255];
    let light_chip = [218, 217, 204, 255];
    let mut config = AnimateConfig::default().with_resolution(16, 16).with_output_dir(&out).with_media_dir(out.join("media"));
    config.cache.partial_movie_dir = out.join("partials");
    config.background = dark.map(|component| f64::from(component) / 255.0);
    config.frame_rate = 15.0;
    let mut writer = SceneFileWriter::new(&config, &[OutputFormat::Mp4, OutputFormat::Gif]).unwrap();
    for (index, (base, chip)) in [(dark, dark_chip), (light, light_chip)].into_iter().enumerate() {
        let mut pixels = base.repeat(256);
        for row in pixels.chunks_exact_mut(16 * 4).skip(4).take(8) {
            row[4 * 4..12 * 4].copy_from_slice(&chip.repeat(8));
        }
        let partial = writer.begin_partial(index as u32 * 2).unwrap();
        writer.push_frame(&pixels, index as u32 * 2).unwrap();
        writer.push_frame(&pixels, index as u32 * 2 + 1).unwrap();
        writer.finalize_partial(&partial).unwrap();
    }
    writer.encode_outputs(None).unwrap();
    let sample = Mp4Sample { data: dark.repeat(256), duration: 1, cts_offset: 0, sync: true };
    let track = Mp4Track { track_id: 1, timescale: 15, width: 16, height: 16, codec: Default::default(), metadata: Default::default(), chunk_sample_counts: vec![1], samples: vec![sample] };
    let broken = Mp4Snapshot { tracks: vec![track], ..Mp4Snapshot::default() };
    fs::write(out.join("raw-avc1.mp4"), encode_mp4(&broken)).unwrap();
}
