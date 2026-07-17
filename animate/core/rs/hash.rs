use crate::config::AnimateConfig;
use crate::frame::FrameSnapshot;
use crate::scene::SceneContext;
use crate::sobject::{MobjectStore, Mobility, Sobject, SobjectShape};
use crate::timeline::TimelineSegment;
use mathematical_geometry::{Affine, Point};
use semio_framework_hash::{format_number_for_hash, hash_parts};

/// 🪪 Hashes the full scene animation definition.
pub fn animation_hash(config: &AnimateConfig, context: &SceneContext) -> String {
    let mut parts = vec![
        config.pixel_width.to_string(),
        config.pixel_height.to_string(),
        format_number_for_hash(config.frame_rate),
        config.file_stem.clone(),
        config.seed.to_string(),
        color_parts(config.background_color),
        format_number_for_hash(context.timeline().total_duration()),
        mobject_store_hash(context.mobjects()),
    ];
    for segment in context.timeline().segments() {
        parts.push(segment_hash(segment));
    }
    hash_parts(&parts)
}

/// 🪪 Hashes one rendered frame.
pub fn frame_hash(frame_index: u32, time: f64, mobjects: &MobjectStore, background: [f32; 4]) -> String {
    hash_parts(&[
        frame_index.to_string(),
        format_number_for_hash(time),
        mobject_store_hash(mobjects),
        color_parts(background),
    ])
}

/// 🪨 Hashes static Sobjects for background reuse.
pub fn static_layer_hash(mobjects: &MobjectStore, background: [f32; 4]) -> String {
    let mut parts = vec![color_parts(background)];
    for sobject in mobjects.static_objects() {
        parts.push(sobject_hash(sobject));
    }
    hash_parts(&parts)
}

fn color_parts(color: [f32; 4]) -> String {
    format!(
        "{}/{}/{}/{}",
        format_number_for_hash(color[0]),
        format_number_for_hash(color[1]),
        format_number_for_hash(color[2]),
        format_number_for_hash(color[3])
    )
}

fn segment_hash(segment: &TimelineSegment) -> String {
    match segment {
        TimelineSegment::Play { start, duration } => {
            format!("play:{}/{}", format_number_for_hash(*start), format_number_for_hash(*duration))
        }
        TimelineSegment::Wait { start, duration } => {
            format!("wait:{}/{}", format_number_for_hash(*start), format_number_for_hash(*duration))
        }
    }
}

fn mobject_store_hash(store: &MobjectStore) -> String {
    let mut parts = Vec::new();
    for sobject in store.sorted() {
        parts.push(sobject_hash(sobject));
    }
    hash_parts(&parts)
}

fn sobject_hash(sobject: &Sobject) -> String {
    hash_parts(&[
        sobject.id.0.to_string(),
        shape_hash(&sobject.shape),
        affine_hash(&sobject.transform),
        paint_hash(sobject.fill.as_ref()),
        stroke_hash(sobject.stroke.as_ref()),
        sobject.z_index.to_string(),
        match sobject.mobility {
            Mobility::Static => "static",
            Mobility::Moving => "moving",
        }
        .into(),
    ])
}

fn paint_hash(paint: Option<&crate::sobject::PaintStyle>) -> String {
    paint.map_or_else(|| "none".into(), |p| color_parts(p.color))
}

fn stroke_hash(stroke: Option<&crate::sobject::StrokeStyle>) -> String {
    stroke.map_or_else(|| "none".into(), |s| format!("{}/{}", color_parts(s.color), format_number_for_hash(s.width)))
}

fn point_hash(point: Point) -> String {
    format!("{}/{}", format_number_for_hash(point.x()), format_number_for_hash(point.y()))
}

fn affine_hash(affine: &Affine) -> String {
    let k = affine.to_kurbo().as_coeffs();
    format!(
        "{}/{}/{}/{}/{}/{}",
        format_number_for_hash(k[0]),
        format_number_for_hash(k[1]),
        format_number_for_hash(k[2]),
        format_number_for_hash(k[3]),
        format_number_for_hash(k[4]),
        format_number_for_hash(k[5])
    )
}

fn shape_hash(shape: &SobjectShape) -> String {
    match shape {
        SobjectShape::Circle { center, radius } => format!("circle:{}/{}", point_hash(*center), format_number_for_hash(*radius)),
        SobjectShape::Rect { rect } => format!(
            "rect:{}/{}/{}/{}",
            format_number_for_hash(rect.x0()),
            format_number_for_hash(rect.y0()),
            format_number_for_hash(rect.x1()),
            format_number_for_hash(rect.y1())
        ),
        SobjectShape::RoundedRect { rect } => format!(
            "rounded:{}/{}/{}/{}",
            format_number_for_hash(rect.rect().x0()),
            format_number_for_hash(rect.rect().y0()),
            format_number_for_hash(rect.rect().x1()),
            format_number_for_hash(rect.rect().y1())
        ),
        SobjectShape::Line { line } => format!("line:{}/{}", point_hash(line.from()), point_hash(line.to())),
        SobjectShape::Arc { arc } => format!(
            "arc:{}/{}/{}/{}/{}/{}",
            point_hash(arc.center()),
            format_number_for_hash(arc.radius()),
            format_number_for_hash(arc.start_angle()),
            format_number_for_hash(arc.sweep_angle()),
            arc.x_rotation().to_string(),
            arc.large_arc().to_string()
        ),
        SobjectShape::Path { path } => format!("path:{}", path.elements().len()),
    }
}
