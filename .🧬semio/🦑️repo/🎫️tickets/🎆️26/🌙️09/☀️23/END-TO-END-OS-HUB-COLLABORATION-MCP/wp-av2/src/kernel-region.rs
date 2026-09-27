//#region 🎞️VideoRenderProgram
/// 🎟️ The host capability a plugin requests (`CapabilityRequest.id` in its package descriptor) before any host renders
/// its [`Effect::VideoRenderExport`]: the host lends its canvas and its encoder only to plugins that declared the ask, and
/// answers every other plugin with the `capability` refusal of `🧫️fixtures/🎞️video-render-job/🔣️.json`.
pub const MEDIA_VIDEO_RENDER_CAPABILITY: &str = "media.video-render";

/// 🎞️ The schema id every [`VideoRenderProgram`] states; `🧫️fixtures/🎞️video-render-program/🔣️.json` is its
/// language-agnostic law, `videoRenderProgramProblem` in `🎠️kernel/🟦️.ts` its TypeScript twin.
pub const VIDEO_RENDER_PROGRAM_SCHEMA: &str = "semio.video-render.program.v1";

/// 📏️ The largest picture edge a program may ask for (the raster video tier's own ceiling).
pub const VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE: u32 = 4096;

/// 🎞️ The fastest frame rate a program may ask for.
pub const VIDEO_RENDER_PROGRAM_MAXIMUM_FPS: u32 = 120;

/// ⏱️ The most frames one program may render (ten minutes at 60 fps).
pub const VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES: u64 = 36_000;

/// ✏️ One path in a program's shared path table: `verbs` is one letter per segment (`M` move, `L` line,
/// `Q` quadratic, `C` cubic, `Z` close), `points` the flat `x, y` pairs they consume (1, 1, 2, 3, 0).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderPath {
    pub verbs: String,
    pub points: Vec<f64>,
}

/// 🖼️ One picture a program draws from: a same-origin absolute path (`/🖼️assets/…`) or a `data:image/…` URL, never a
/// foreign origin — a host fetches it itself, and a cross-origin picture would taint the canvas it encodes from.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderImage {
    pub url: String,
}

/// 🎨️ One paint operation of a frame, composited in list order over the program background. Every `transform` is
/// `[a, b, c, d, e, f]` (`x' = a x + c y + e`, `y' = b x + d y + f`) into device pixels, y down; every colour is straight
/// RGBA in `0..=1`. `Fill`/`Stroke` paint path `path` of the table (a stroke `width` path units wide); `Image` draws the
/// `crop` (`[x, y, width, height]`, normalised to the picture) of image `image` onto the unit square that `transform`
/// places, at `opacity`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VideoRenderOp {
    Fill { path: u32, transform: [f64; 6], color: [f64; 4] },
    Stroke { path: u32, transform: [f64; 6], color: [f64; 4], width: f64 },
    Image { image: u32, crop: [f64; 4], transform: [f64; 6], opacity: f64 },
}

/// 🖼️ One distinct picture: its paint operations over the program background.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderScene {
    pub ops: Vec<VideoRenderOp>,
}

/// ⏯️ Scene `scene` shown for `frames` consecutive frames.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderRun {
    pub scene: u32,
    pub frames: u32,
}

/// 🎞️ Everything a host needs to render a video without asking the guest again: picture size, frame rate,
/// background, a shared path table, the pictures it draws from, the distinct scenes and the run-length timeline that
/// plays them. Identical consecutive frames are one run, so a still slide costs one scene however long it plays.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderProgram {
    pub schema: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub background: [f64; 4],
    pub paths: Vec<VideoRenderPath>,
    pub images: Vec<VideoRenderImage>,
    pub scenes: Vec<VideoRenderScene>,
    pub timeline: Vec<VideoRenderRun>,
}

/// 🚨️ Why a host refuses a [`VideoRenderProgram`]; `code()` is the fixture's language-agnostic vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VideoRenderProgramError {
    Schema { schema: String },
    Dimensions { width: u32, height: u32 },
    FrameRate { fps: u32 },
    Empty,
    TooLong { frames: u64 },
    Timeline { run: usize },
    PathVerbs { path: usize },
    ImageUrl { image: usize },
    PathIndex { scene: usize, op: usize },
    ImageIndex { scene: usize, op: usize },
    Paint { scene: usize, op: usize },
}

impl VideoRenderProgramError {
    /// 🔤️ The fixture's error code for this refusal.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Schema { .. } => "schema",
            Self::Dimensions { .. } => "dimensions",
            Self::FrameRate { .. } => "frameRate",
            Self::Empty => "empty",
            Self::TooLong { .. } => "tooLong",
            Self::Timeline { .. } => "timeline",
            Self::PathVerbs { .. } => "pathVerbs",
            Self::ImageUrl { .. } => "imageUrl",
            Self::PathIndex { .. } => "pathIndex",
            Self::ImageIndex { .. } => "imageIndex",
            Self::Paint { .. } => "paint",
        }
    }
}

impl std::fmt::Display for VideoRenderProgramError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Schema { schema } => write!(formatter, "video program schema {schema:?} is not {VIDEO_RENDER_PROGRAM_SCHEMA}"),
            Self::Dimensions { width, height } => write!(formatter, "video program picture {width}x{height} is not an even size within 2..={VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE}"),
            Self::FrameRate { fps } => write!(formatter, "video program frame rate {fps} is outside 1..={VIDEO_RENDER_PROGRAM_MAXIMUM_FPS}"),
            Self::Empty => formatter.write_str("video program has no frames"),
            Self::TooLong { frames } => write!(formatter, "video program has {frames} frames, more than {VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES}"),
            Self::Timeline { run } => write!(formatter, "video program timeline run {run} names no scene or shows it for zero frames"),
            Self::PathVerbs { path } => write!(formatter, "video program path {path} has verbs its points do not match"),
            Self::ImageUrl { image } => write!(formatter, "video program image {image} is neither a same-origin path nor a data:image URL"),
            Self::PathIndex { scene, op } => write!(formatter, "video program scene {scene} op {op} names no path"),
            Self::ImageIndex { scene, op } => write!(formatter, "video program scene {scene} op {op} names no image"),
            Self::Paint { scene, op } => write!(formatter, "video program scene {scene} op {op} has a colour, transform, width, crop or opacity out of range"),
        }
    }
}

impl std::error::Error for VideoRenderProgramError {}

impl VideoRenderProgram {
    /// 🎞️ Frames the timeline plays.
    pub fn frame_count(&self) -> u64 {
        self.timeline.iter().map(|run| u64::from(run.frames)).sum()
    }

    /// ⏱️ Playing time in milliseconds, rounded half up — what the encoded MP4 states.
    pub fn duration_milliseconds(&self) -> u64 {
        (self.frame_count() * 1000 + u64::from(self.fps) / 2) / u64::from(self.fps.max(1))
    }

    /// 🚦️ The ONE admission rule every host applies before it renders a frame.
    pub fn validate(&self) -> Result<(), VideoRenderProgramError> {
        if self.schema != VIDEO_RENDER_PROGRAM_SCHEMA {
            return Err(VideoRenderProgramError::Schema { schema: self.schema.clone() });
        }
        let edge = |value: u32| (2..=VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE).contains(&value) && value % 2 == 0;
        if !edge(self.width) || !edge(self.height) {
            return Err(VideoRenderProgramError::Dimensions { width: self.width, height: self.height });
        }
        if self.fps == 0 || self.fps > VIDEO_RENDER_PROGRAM_MAXIMUM_FPS {
            return Err(VideoRenderProgramError::FrameRate { fps: self.fps });
        }
        let frames = self.frame_count();
        if frames == 0 {
            return Err(VideoRenderProgramError::Empty);
        }
        if frames > VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES {
            return Err(VideoRenderProgramError::TooLong { frames });
        }
        if let Some(run) = self.timeline.iter().position(|run| run.frames == 0 || run.scene as usize >= self.scenes.len()) {
            return Err(VideoRenderProgramError::Timeline { run });
        }
        if let Some(path) = self.paths.iter().position(|path| !video_render_path_is_well_formed(path)) {
            return Err(VideoRenderProgramError::PathVerbs { path });
        }
        if let Some(image) = self.images.iter().position(|image| !video_render_image_url_is_admitted(&image.url)) {
            return Err(VideoRenderProgramError::ImageUrl { image });
        }
        if !self.background.iter().all(|value| video_render_unit(*value)) {
            return Err(VideoRenderProgramError::Paint { scene: 0, op: 0 });
        }
        let finite = |values: &[f64]| values.iter().all(|value| value.is_finite());
        for (scene_index, scene) in self.scenes.iter().enumerate() {
            for (op_index, op) in scene.ops.iter().enumerate() {
                let (reference, painted) = match op {
                    VideoRenderOp::Fill { path, transform, color } => ((*path as usize) < self.paths.len(), finite(transform) && color.iter().all(|value| video_render_unit(*value))),
                    VideoRenderOp::Stroke { path, transform, color, width } => ((*path as usize) < self.paths.len(), finite(transform) && color.iter().all(|value| video_render_unit(*value)) && width.is_finite() && *width >= 0.0),
                    VideoRenderOp::Image { image, crop, transform, opacity } => ((*image as usize) < self.images.len(), finite(transform) && video_render_crop_is_admitted(crop) && video_render_unit(*opacity)),
                };
                if !reference {
                    return Err(match op {
                        VideoRenderOp::Image { .. } => VideoRenderProgramError::ImageIndex { scene: scene_index, op: op_index },
                        _ => VideoRenderProgramError::PathIndex { scene: scene_index, op: op_index },
                    });
                }
                if !painted {
                    return Err(VideoRenderProgramError::Paint { scene: scene_index, op: op_index });
                }
            }
        }
        Ok(())
    }
}

fn video_render_unit(value: f64) -> bool {
    value.is_finite() && (0.0..=1.0).contains(&value)
}

/// ✏️ A path's verbs consume exactly its points, start with a move and every coordinate is finite.
pub fn video_render_path_is_well_formed(path: &VideoRenderPath) -> bool {
    let mut needed = 0usize;
    for (index, verb) in path.verbs.chars().enumerate() {
        needed += match verb {
            'M' => 2,
            'L' => 2,
            'Q' => 4,
            'C' => 6,
            'Z' => 0,
            _ => return false,
        };
        if index == 0 && verb != 'M' {
            return false;
        }
    }
    needed == path.points.len() && path.points.iter().all(|value| value.is_finite())
}

/// 🖼️ An image URL a host may fetch for a program: a same-origin absolute path (`/…`, never the protocol-relative
/// `//…`) or an inline `data:image/…` URL.
pub fn video_render_image_url_is_admitted(url: &str) -> bool {
    (url.starts_with('/') && !url.starts_with("//")) || url.starts_with("data:image/")
}

/// ✂️ A crop `[x, y, width, height]` lies inside the unit picture and covers a non-empty area.
pub fn video_render_crop_is_admitted(crop: &[f64; 4]) -> bool {
    let [x, y, width, height] = *crop;
    crop.iter().all(|value| value.is_finite()) && x >= 0.0 && y >= 0.0 && width > 0.0 && height > 0.0 && x + width <= 1.0 + 1e-9 && y + height <= 1.0 + 1e-9
}

#[cfg(test)]
#[path = "🧪️tests/🎞️video-render-program/🦀️.rs"]
mod video_render_program_tests;
//#endregion 🎞️VideoRenderProgram

//#region 🧵️VideoRenderJob
/// 🧩️ Which encoder produced a finished render: the host's platform encoder (the OS or browser H.264 encoder plugged in
/// behind the raster video tier's port) or the tier's own first-party encoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum VideoRenderEncoderTier {
    Platform,
    FirstParty,
}

/// 🏁️ How one host video render job ended.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "status", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "status", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VideoRenderJobOutcome {
    Done { bytes: u64, tier: VideoRenderEncoderTier },
    Cancelled { completed: u64 },
    Refused { code: String },
    Failed { reason: String },
}

/// 🧵️ One fact of a host video render job's life — ephemeral, local-only state of the host that renders it. A host
/// never mutates its task list: it appends one of these and the task list is the fold of the log
/// ([`VideoRenderJobLedger`]); a cancel is a command that appends [`Self::CancelRequested`], which the render loop reads
/// back from the ledger before its next frame. `🧫️fixtures/🎞️video-render-job/🔣️.json` is the law both twins fold.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VideoRenderJobEvent {
    Started { job: u64, owner: String, filename: String, frames: u64, at_ms: u64 },
    Progressed { job: u64, completed: u64 },
    CancelRequested { job: u64 },
    Finished { job: u64, outcome: VideoRenderJobOutcome },
}

/// 🧵️ One running job as a task list shows it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRenderJobRow {
    pub job: u64,
    pub owner: String,
    pub filename: String,
    pub frames: u64,
    pub completed: u64,
    pub cancelling: bool,
    pub started_at_ms: u64,
}

/// 🚨️ Why a [`VideoRenderJobLedger`] refuses an event; `code()` is the fixture's vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VideoRenderJobEventError {
    StaleJob { job: u64, last: u64 },
    UnknownJob { job: u64 },
    ProgressRegressed { job: u64, completed: u64, previous: u64 },
    ProgressOverrun { job: u64, completed: u64, frames: u64 },
    AlreadyCancelling { job: u64 },
}

impl VideoRenderJobEventError {
    /// 🔤️ The fixture's error code for this refusal.
    pub fn code(&self) -> &'static str {
        match self {
            Self::StaleJob { .. } => "staleJob",
            Self::UnknownJob { .. } => "unknownJob",
            Self::ProgressRegressed { .. } => "progressRegressed",
            Self::ProgressOverrun { .. } => "progressOverrun",
            Self::AlreadyCancelling { .. } => "alreadyCancelling",
        }
    }
}

impl std::fmt::Display for VideoRenderJobEventError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaleJob { job, last } => write!(formatter, "video render job {job} does not follow job {last}"),
            Self::UnknownJob { job } => write!(formatter, "video render job {job} is not running"),
            Self::ProgressRegressed { job, completed, previous } => write!(formatter, "video render job {job} went back from frame {previous} to {completed}"),
            Self::ProgressOverrun { job, completed, frames } => write!(formatter, "video render job {job} reports frame {completed} of {frames}"),
            Self::AlreadyCancelling { job } => write!(formatter, "video render job {job} is already cancelling"),
        }
    }
}

impl std::error::Error for VideoRenderJobEventError {}

/// 📒️ The fold of a host's [`VideoRenderJobEvent`] log: the running jobs in start order and the last job id issued.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VideoRenderJobLedger {
    last_job: u64,
    running: Vec<VideoRenderJobRow>,
}

impl VideoRenderJobLedger {
    /// 📒️ Folds a whole log; the error names the index of the first event the ledger refused.
    pub fn fold<'a>(events: impl IntoIterator<Item = &'a VideoRenderJobEvent>) -> Result<Self, (usize, VideoRenderJobEventError)> {
        let mut ledger = Self::default();
        for (index, event) in events.into_iter().enumerate() {
            ledger.apply(event).map_err(|error| (index, error))?;
        }
        Ok(ledger)
    }

    /// ➕️ Applies one event, or refuses it and leaves the ledger as it was.
    pub fn apply(&mut self, event: &VideoRenderJobEvent) -> Result<(), VideoRenderJobEventError> {
        match event {
            VideoRenderJobEvent::Started { job, owner, filename, frames, at_ms } => {
                if *job <= self.last_job {
                    return Err(VideoRenderJobEventError::StaleJob { job: *job, last: self.last_job });
                }
                self.last_job = *job;
                self.running.push(VideoRenderJobRow { job: *job, owner: owner.clone(), filename: filename.clone(), frames: *frames, completed: 0, cancelling: false, started_at_ms: *at_ms });
            }
            VideoRenderJobEvent::Progressed { job, completed } => {
                let row = self.running.iter_mut().find(|row| row.job == *job).ok_or(VideoRenderJobEventError::UnknownJob { job: *job })?;
                if *completed < row.completed {
                    return Err(VideoRenderJobEventError::ProgressRegressed { job: *job, completed: *completed, previous: row.completed });
                }
                if *completed > row.frames {
                    return Err(VideoRenderJobEventError::ProgressOverrun { job: *job, completed: *completed, frames: row.frames });
                }
                row.completed = *completed;
            }
            VideoRenderJobEvent::CancelRequested { job } => {
                let row = self.running.iter_mut().find(|row| row.job == *job).ok_or(VideoRenderJobEventError::UnknownJob { job: *job })?;
                if row.cancelling {
                    return Err(VideoRenderJobEventError::AlreadyCancelling { job: *job });
                }
                row.cancelling = true;
            }
            VideoRenderJobEvent::Finished { job, .. } => {
                let index = self.running.iter().position(|row| row.job == *job).ok_or(VideoRenderJobEventError::UnknownJob { job: *job })?;
                self.running.remove(index);
            }
        }
        Ok(())
    }

    /// 🧵️ Every running job, in start order.
    pub fn running(&self) -> &[VideoRenderJobRow] {
        &self.running
    }

    /// 🔎️ Running job `job`, if any.
    pub fn row(&self, job: u64) -> Option<&VideoRenderJobRow> {
        self.running.iter().find(|row| row.job == job)
    }

    /// 🔢️ The last job id issued (`0` before the first).
    pub fn last_job(&self) -> u64 {
        self.last_job
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧵️video-render-job/🦀️.rs"]
mod video_render_job_tests;
//#endregion 🧵️VideoRenderJob
