fn image_header(projected: &PdfImage, current: &PdfImage) -> bool {
    let supported = |image: &PdfImage| {
        image.id.len() <= TEXT_CAP && image.bits_per_component == 8
            && matches!(image.color_space, Some(PdfColorSpace::DeviceGray | PdfColorSpace::DeviceRgb))
            && !image.image_mask && image.decode.is_empty() && image.soft_mask.is_none()
            && image.soft_mask_in_data.is_none() && image.mask.is_none() && image.matte.is_none()
            && image.intent.is_none() && image.optional_content.is_none() && image.struct_parent.is_none()
            && image.extra.is_empty()
    };
    supported(projected) && supported(current) && projected.id == current.id
        && (projected.width, projected.height, projected.interpolate) == (current.width, current.height, current.interpolate)
        && projected.color_space == current.color_space
}

fn compare_image(projected: &PdfImage, current: &PdfImage, offset: &mut usize) -> Result<bool, ExactGraphRefusal> {
    if *offset == 0 && !image_header(projected, current) { return Err(ExactGraphRefusal::Semantic); }
    match (&projected.body, &current.body) {
        (PdfImageBody::Artifact { reference: left }, PdfImageBody::Artifact { reference: right }) if left == right => Ok(true),
        (PdfImageBody::Samples { values: left }, PdfImageBody::Samples { values: right }) if left.len() == right.len() => {
            let end = left.len().min(offset.saturating_add(COMPARE_WORK / size_of::<u32>()));
            if left[*offset..end] != right[*offset..end] { return Err(ExactGraphRefusal::Semantic); }
            *offset = end;
            Ok(end == left.len())
        }
        _ => Err(ExactGraphRefusal::Semantic),
    }
}
