//#region 🔖️Error
/// 🎬️ Why a presentation deck or scene cannot become a host video program (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
/// split from the former engine-tree `PresentationError`; the schema-tier half is
/// `crate::standards::v1::subsets::any::schema::PresentationError`).
#[derive(Debug, PartialEq, Eq)]
pub enum PresentationVideoExportError {
    /// 🎬️ The scene had no scene hashes to render.
    NoSceneHashes,
    /// 📄️ The deck's source is not a picture a host canvas can draw (a PDF page, or no source at all).
    SourceKind { kind: String },
    /// 🚦️ The program would be refused by every host (`VideoRenderProgram::validate`).
    Program(semio_framework::kernel::VideoRenderProgramError),
}

impl PresentationVideoExportError {
    /// 🔤️ The fault code a command answers this error with.
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoSceneHashes => "animate.video.export.no-scenes",
            Self::SourceKind { .. } => "animate.video.export.source-kind",
            Self::Program(_) => "animate.video.export.program",
        }
    }
}

impl std::fmt::Display for PresentationVideoExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSceneHashes => formatter.write_str("presentation has no scene hashes to export"),
            Self::SourceKind { kind } => write!(formatter, "the deck's source ({kind:?}) is not a picture a video can show"),
            Self::Program(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for PresentationVideoExportError {}
//#endregion 🔖️Error

//#region 🔖️VideoExport
/// ⏱️ How long the opening overview (the whole source with every tile outlined) plays.
pub const DECK_VIDEO_OVERVIEW_SECONDS: f64 = 3.0;

/// ⏱️ How long each tile's slide plays.
pub const DECK_VIDEO_SLIDE_SECONDS: f64 = 2.0;

/// 📐️ The share of the picture's shorter edge kept free around every slide.
pub const DECK_VIDEO_MARGIN: f64 = 0.05;

/// ✏️ The stroke width, in device pixels, of a tile outline on the overview.
pub const DECK_VIDEO_OUTLINE_PIXELS: f64 = 3.0;

/// 🎨️ The tile outline colour on the overview (straight RGBA).
pub const DECK_VIDEO_OUTLINE_COLOR: [f64; 4] = [1.0, 0.78, 0.2, 1.0];

/// 🎞️ The host video program of a tile deck at the Medium preset: an overview of the whole source picture with every tile
/// outlined, then one slide per tile showing the tile's crop of the picture, each fitted into the frame (aspect kept, a
/// margin around it). A still slide is one scene however long it plays, so the program stays a few kilobytes and the
/// host encodes one picture per slide. The picture is drawn by the host from `source.src`; nothing here decodes it.
pub fn video_render_program_from_deck(deck: &crate::PresentationSnapshot) -> Result<semio_framework::kernel::VideoRenderProgram, PresentationVideoExportError> {
    use crate::editor::animate::engine::config::config::{AnimateConfig, QualityPreset};
    use semio_framework::kernel::{VideoRenderImage, VideoRenderOp, VideoRenderPath, VideoRenderProgram, VideoRenderRun, VideoRenderScene, VIDEO_RENDER_PROGRAM_SCHEMA};
    let (source, tiles) = crate::presentation_working_scene(deck);
    if source.src.trim().is_empty() || source.kind == "pdf" {
        return Err(PresentationVideoExportError::SourceKind { kind: source.kind.clone() });
    }
    let config = AnimateConfig::from_quality(QualityPreset::Medium);
    let (width, height) = (f64::from(config.width), f64::from(config.height));
    let fps = config.frame_rate.round().max(1.0) as u32;
    let frames = |seconds: f64| (seconds * f64::from(fps)).round().max(1.0) as u32;
    let frame = &source.frame;
    let aspect = source.source_aspect.filter(|aspect| aspect.is_finite() && *aspect > 0.0).unwrap_or(if frame.height > 0.0 { frame.width / frame.height } else { 1.0 });
    let normalised = |value: f64, origin: f64, extent: f64| if extent > 0.0 { ((value - origin) / extent).clamp(0.0, 1.0) } else { 0.0 };
    let margin = width.min(height) * DECK_VIDEO_MARGIN;
    let fit = |crop: [f64; 4]| {
        let picture_aspect = crop[2] * aspect / crop[3].max(f64::EPSILON);
        let (room_width, room_height) = (width - 2.0 * margin, height - 2.0 * margin);
        let (fit_width, fit_height) = if picture_aspect >= room_width / room_height { (room_width, room_width / picture_aspect) } else { (room_height * picture_aspect, room_height) };
        [fit_width, 0.0, 0.0, fit_height, (width - fit_width) / 2.0, (height - fit_height) / 2.0]
    };
    let crops: Vec<[f64; 4]> = tiles
        .iter()
        .map(|tile| {
            let (x, y) = (normalised(tile.crop.x, frame.x, frame.width), normalised(tile.crop.y, frame.y, frame.height));
            let (right, bottom) = (normalised(tile.crop.x + tile.crop.width, frame.x, frame.width), normalised(tile.crop.y + tile.crop.height, frame.y, frame.height));
            [x, y, right - x, bottom - y]
        })
        .filter(|crop| crop[2] > 0.0 && crop[3] > 0.0)
        .collect();
    let overview = fit([0.0, 0.0, 1.0, 1.0]);
    let outline = |crop: &[f64; 4]| {
        let (left, top) = (overview[4] + crop[0] * overview[0], overview[5] + crop[1] * overview[3]);
        let (right, bottom) = (left + crop[2] * overview[0], top + crop[3] * overview[3]);
        VideoRenderPath { verbs: "MLLLZ".into(), points: vec![left, top, right, top, right, bottom, left, bottom] }
    };
    let identity = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
    let mut overview_ops = vec![VideoRenderOp::Image { image: 0, crop: [0.0, 0.0, 1.0, 1.0], transform: overview, opacity: 1.0 }];
    overview_ops.extend((0..crops.len()).map(|index| VideoRenderOp::Stroke { path: index as u32, transform: identity, color: DECK_VIDEO_OUTLINE_COLOR, width: DECK_VIDEO_OUTLINE_PIXELS }));
    let mut scenes = vec![VideoRenderScene { ops: overview_ops }];
    scenes.extend(crops.iter().map(|crop| VideoRenderScene { ops: vec![VideoRenderOp::Image { image: 0, crop: *crop, transform: fit(*crop), opacity: 1.0 }] }));
    let mut timeline = vec![VideoRenderRun { scene: 0, frames: frames(DECK_VIDEO_OVERVIEW_SECONDS) }];
    timeline.extend((1..scenes.len()).map(|scene| VideoRenderRun { scene: scene as u32, frames: frames(DECK_VIDEO_SLIDE_SECONDS) }));
    let program = VideoRenderProgram {
        schema: VIDEO_RENDER_PROGRAM_SCHEMA.into(),
        width: config.width,
        height: config.height,
        fps,
        background: config.background.map(|value| if value.is_nan() { 0.0 } else { value.clamp(0.0, 1.0) }),
        paths: crops.iter().map(outline).collect(),
        images: vec![VideoRenderImage { url: source.src.clone() }],
        scenes,
        timeline,
    };
    program.validate().map_err(PresentationVideoExportError::Program)?;
    Ok(program)
}

/// 🎬️ The host video program of every unique `scene_hash` a {@link PresentationScene} references, in slide order,
/// at the Medium preset: each hash's scene is played and captured target-neutrally (`video::capture_scene`) and every
/// captured frame joins one program (`video::VideoProgramBuilder`). The host renders + encodes it
/// (`Effect::VideoRenderExport`); nothing here touches a GPU or a file system.
pub fn video_render_program_from_scene(scene: &PresentationScene) -> Result<semio_framework::kernel::VideoRenderProgram, PresentationVideoExportError> {
    use crate::editor::animate::engine::config::config::{AnimateConfig, QualityPreset};
    let hashes = scene.scene_hashes();
    if hashes.is_empty() {
        return Err(PresentationVideoExportError::NoSceneHashes);
    }
    let config = AnimateConfig::from_quality(QualityPreset::Medium);
    let mut builder = crate::editor::animate::engine::video::VideoProgramBuilder::new(&config);
    for hash in hashes {
        let capture = crate::editor::animate::engine::video::capture_scene(crate::editor::animate::engine::video::scene_for_hash(config.clone(), &hash), &config);
        for frame in &capture.captures {
            builder.push_capture(frame, &capture.camera, &config);
        }
    }
    let program = builder.finish();
    program.validate().map_err(PresentationVideoExportError::Program)?;
    Ok(program)
}

/// 📛️ A download file name from `title`: its ASCII letters and digits as a lowercase slug (`animate-presentation` when
/// nothing is left).
pub fn video_filename_for_title(title: &str) -> String {
    let slug: String = title.chars().map(|character| if character.is_ascii_alphanumeric() { character.to_ascii_lowercase() } else { '-' }).collect();
    let slug = slug.split('-').filter(|part| !part.is_empty()).collect::<Vec<_>>().join("-");
    format!("{}.mp4", if slug.is_empty() { "animate-presentation" } else { slug.as_str() })
}

/// 📛️ The downloaded file name of a deck's video: its source picture's file stem (`/…/🏘️habitat-67.png` → `habitat-67.mp4`).
pub fn video_filename_for_deck(deck: &crate::PresentationSnapshot) -> String {
    let (source, _) = crate::presentation_working_scene(deck);
    let stem = if source.src.starts_with("data:") { "" } else { source.src.rsplit('/').next().unwrap_or("").rsplit_once('.').map_or(source.src.rsplit('/').next().unwrap_or(""), |(stem, _)| stem) };
    video_filename_for_title(stem)
}
//#endregion 🔖️VideoExport
