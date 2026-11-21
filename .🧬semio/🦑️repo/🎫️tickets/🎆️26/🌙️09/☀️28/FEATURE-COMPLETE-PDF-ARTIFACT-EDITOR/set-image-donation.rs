use crate::standards::v1_7::subsets::base::schema::snapshot::PdfImageBody;

const SAMPLE_PATH: [&str; 3] = ["image", "body", "values"];
const SAMPLE_TAGS: [(usize, &str); 2] = [(0, "mutation"), (2, "kind")];
const SAMPLE_BACKING_BYTES: usize = 1 << 20;

fn donate_samples(mutation: &mut PdfMutation, donor: &mut Option<Vec<u32>>) -> Result<(), ValueError> {
    let Some(_) = donor else { return Ok(()); };
    let PdfMutation::SetImage(operation) = mutation else {
        return Err(ValueError::invariant("PDF sample donor changed mutation family"));
    };
    let PdfImageBody::Samples { values } = &mut operation.image.body else {
        return Err(ValueError::invariant("PDF sample donor changed image body family"));
    };
    if !values.is_empty() || values.capacity() != 0 {
        return Err(ValueError::invariant("PDF sample donor destination is occupied"));
    }
    *values = donor.take().expect("validated PDF sample donor");
    Ok(())
}

impl PdfMutationReceiving {
    fn samples_start(&self, source: &[u8]) -> Result<Option<usize>, ValueError> {
        if self.frame.kind != Some("setImage") || self.samples.value.is_some() {
            return Ok(None);
        }
        let selected = self.parser.value.as_ref().and_then(|parser|
            parser.selected_number_array_tags(self.frame.payload(source), &SAMPLE_PATH, &SAMPLE_TAGS));
        let Some((start, [mutation, body])) = selected else { return Ok(None); };
        if mutation.is_some_and(|tag| tag.as_str() != Some("setImage")) {
            return Err(receiving_invalid("PDF mutation tag differs from its JSON identity"));
        }
        match body.map(DslValue::as_str) {
            Some(Some("samples")) if mutation.is_some() => Ok(Some(start)),
            Some(Some("samples" | "artifact")) | None => Ok(None),
            _ => Err(receiving_invalid("unknown PDF image body discriminator")),
        }
    }

    fn advance_samples(&mut self, source: &[u8], effect: &mut RetainedAllocationEffect<'_>)
        -> Result<RetainedCloneProgress, ValueError> {
        let payload = self.frame.payload(source);
        let cursor = self.sample_cursor.as_mut().expect("admitted PDF Samples cursor");
        match self.phase {
            ReceivingPhase::Samples => {
                let progress = cursor.advance(payload, effect, || true).result?;
                if cursor.is_ready() { self.phase = ReceivingPhase::SamplesMarker; }
                Ok(progress)
            }
            ReceivingPhase::SamplesMarker => {
                let span = cursor.finished_span(payload)?.expect("validated PDF Samples span");
                let parser = self.parser.value.as_mut().expect("admitted PDF mutation JSON parser");
                let progress = parser.accept_number_array_marker(payload, &SAMPLE_PATH, &span, effect)?;
                if progress.copied_items != 0 { self.phase = ReceivingPhase::SamplesTake; }
                Ok(progress)
            }
            ReceivingPhase::SamplesTake => {
                let progress = cursor.take(&mut self.samples.value, effect)?;
                if self.samples.value.is_some() {
                    self.sample_cursor = None;
                    self.phase = ReceivingPhase::Json;
                }
                Ok(progress)
            }
            _ => unreachable!(),
        }
    }

    fn bind_mutation(&mut self) -> Result<(), ValueError> {
        let value = self.value.value.take().expect("admitted PDF mutation JSON value");
        let mutation = PdfMutation::from_value(value)?;
        *self.output.value = Some(mutation);
        donate_samples(self.output.value.as_mut().expect("resident PDF mutation output"), &mut self.samples.value)?;
        self.phase = ReceivingPhase::Ready;
        Ok(())
    }
}
