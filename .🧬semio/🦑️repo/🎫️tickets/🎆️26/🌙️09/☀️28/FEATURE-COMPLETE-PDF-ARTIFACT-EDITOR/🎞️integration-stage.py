"""🛠️ Stages PDF image admission integration against the current native source."""

from pathlib import Path
import sys

LIFT = Path("🔨️modules/⬇️lift/🧩️lifter-and-document")
DOCUMENT = LIFT / "🧭️document-lift/🧭️controlled/🦀️.rs"
PROGRAM = LIFT / "📜️page-content/🧺️program/🦀️.rs"
IMPORT = Path("🚪️io/📥️import/🔁️controlled/🦀️.rs")


def replace(text, old, new):
    assert text.count(old) == 1, old
    return text.replace(old, new)


def document(text):
    text = text.replace("vector content, standard Type1 text and simple annotations", "vector content, standard Type1 text, sampled images and simple annotations")
    text = text.replace("Lifts ordinary vector and standard Type1 text pages", "Lifts qualified vector, text and sampled-image pages")
    text = text.replace("Quotes when the next step may reserve empty page, role or annotation slots.", "Quotes empty table or sample backing for the next work unit.")
    text = replace(text, "content::{bounded_number, FontTable}, fonts::{standard_font, FontCodec},", """content::{bounded_number, FontTable}, fonts::{standard_font, FontCodec},
    images::{ControlledImageBinding, ControlledImageResources}, lexer::{CosReceipt, CosStatus},""")
    text = replace(text, "    Font { resources: ObjRef", "    Image(Resume),\n    Font { resources: ObjRef")
    text = replace(text, "    page_font: Option<(ObjRef, usize)>,", "    page_font: Option<(ObjRef, usize)>,\n    images: ControlledImageResources,")
    text = replace(text, "font_owners: [ObjRef::default(); MAX_FONTS], page_font: None,", "font_owners: [ObjRef::default(); MAX_FONTS], page_font: None, images: Default::default(),")
    text = replace(text, """    pub fn reserving_empty_slots(&self) -> bool { matches!(self.stage, Stage::PageCapacity | Stage::AnnotationArray(_, 0, _)) }

    /// 📨️ Quotes the inline annotation body before it is written into an admitted page slot.
    pub fn next_annotation_body_bytes(&self) -> usize { if matches!(self.stage, Stage::Annotation(..)) { size_of::<PdfAnnotation>() } else { 0 } }""", """    pub fn reserving_empty_slots(&self) -> bool {
        matches!(self.stage, Stage::PageCapacity | Stage::AnnotationArray(_, 0, _))
            || matches!(self.stage, Stage::Image(_)) && self.images.reserves_empty(self.snapshot.as_ref().expect("active PDF image snapshot"))
    }

    /// 📨️ Quotes a typed body before insertion into an admitted slot.
    pub fn next_inline_body_bytes(&self) -> usize {
        match self.stage {
            Stage::Annotation(..) => size_of::<PdfAnnotation>(),
            Stage::Image(_) => self.images.inline_body_bytes(self.snapshot.as_ref().expect("active PDF image snapshot")),
            _ => 0,
        }
    }""")
    text = replace(text, "            Stage::Font { resources", "            Stage::Image(resume) => self.image(source, grant, resume),\n            Stage::Font { resources")
    text = replace(text, "        if let Some(walk) = self.walk.as_mut() { walk.begin_close(); }", "        self.images.begin_close();\n        if let Some(walk) = self.walk.as_mut() { walk.begin_close(); }")
    text = replace(text, """        if let Some(program) = self.program.as_mut() {
            let mut turn = content_turn(program.close_step(grant));""", """        let image = self.images.close_step(grant);
        if image.status != Ok(CosStatus::Closed) { return image_turn(image); }
        if let Some(program) = self.program.as_mut() {
            let mut turn = content_turn(program.close_step(grant));""")
    text = replace(text, "        let bytes = count * (size_of::<PdfPage>() + size_of::<PdfAdmittedStreamRole>());", """        let roles = count + ControlledImageResources::MAX_COUNT;
        let bytes = count * size_of::<PdfPage>() + roles * size_of::<PdfAdmittedStreamRole>();""")
    text = replace(text, "snapshot.admitted_stream_roles.try_reserve_exact(count)", "snapshot.admitted_stream_roles.try_reserve_exact(roles)")
    text = replace(text, "                self.page_font = None;", "                self.page_font = None;\n                self.images.set_page(None);")
    text = replace(text, "inspection = Some((empty_dictionary(value), value.and_then(PdfObject::as_ref)));", "inspection = Some((empty_dictionary(value), value.and_then(PdfObject::as_ref), ControlledImageBinding::from_resources(value)));")
    text = replace(text, """            Some((true, _)) => self.resume(resume),
            Some((false, Some(owner))) => self.stage = Stage::ResourceOwner { owner, resume },""", """            Some((true, _, _)) => self.resume(resume),
            Some((false, Some(owner), _)) => self.stage = Stage::ResourceOwner { owner, resume },
            Some((false, None, Some(binding))) => self.begin_image(binding, resume),""")
    text = replace(text, """        let mut font = None;
        let mut empty = false;""", """        let mut font = None;
        let mut image = None;
        let mut empty = false;""")
    text = replace(text, """            font = font_entry(value).and_then(|entry| entry.value.as_ref());""", """            font = font_entry(value).and_then(|entry| entry.value.as_ref());
            image = ControlledImageBinding::from_resources(value);""")
    text = replace(text, """        if resume == Resume::Role {
            if self.page_font.map(|(resources, _)| resources) != font.map(|_| owner) || !empty && font.is_none() {""", """        if let Some(binding) = image {
            if resume == Resume::Role {
                if !self.images.matches_page(self.snapshot.as_ref().expect("active PDF image snapshot"), &binding) { return self.defer(Barrier::Resources); }
                self.resume(resume);
            } else { self.begin_image(binding, resume); }
            return work(0, 0);
        }
        if resume == Resume::Role {
            if self.page_font.map(|(resources, _)| resources) != font.map(|_| owner) || !empty && font.is_none() {""")
    text = replace(text, "    fn font(&mut self, source:", """    fn begin_image(&mut self, binding: ControlledImageBinding, resume: Resume) {
        self.images.begin(binding);
        self.stage = Stage::Image(resume);
    }

    fn image(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, resume: Resume) -> ControlledLiftReceipt {
        let Some(turn) = self.images.advance(source, self.snapshot.as_mut().expect("active PDF image snapshot"), grant) else { return self.defer(Barrier::Resources); };
        if let Ok(CosStatus::Complete { end }) = turn.status {
            if resume == Resume::Inherit { self.images.set_page(Some(end)); }
            self.resume(resume);
        }
        image_turn(turn)
    }

    fn font(&mut self, source:""")
    text = replace(text, "        if depth >= 64 { return self.defer(Barrier::Roles); }\n        let mut step = None;", """        if depth >= 64 { return self.defer(Barrier::Roles); }
        let snapshot = self.snapshot.as_ref().expect("active PDF image snapshot");
        let no_binding = self.page_font.is_none() && self.images.page_name(snapshot).is_none();
        let mut step = None;""")
    text = replace(text, """                Some(value) if empty_dictionary(Some(value)) && self.page_font.is_none() => Some(Stage::RoleStart),
                Some(value) => value.as_ref().map(|owner| Stage::ResourceOwner { owner, resume: Resume::Role }),
                None => dict_get(dict, "Parent").and_then(PdfObject::as_ref).map(|owner| Stage::Ancestor { owner, depth: depth + 1 }).or_else(|| self.page_font.is_none().then_some(Stage::RoleStart)),""", """                Some(value) if empty_dictionary(Some(value)) && no_binding => Some(Stage::RoleStart),
                Some(value) if ControlledImageBinding::from_resources(Some(value)).is_some_and(|binding| self.images.matches_page(snapshot, &binding)) => Some(Stage::RoleStart),
                Some(value) => value.as_ref().map(|owner| Stage::ResourceOwner { owner, resume: Resume::Role }),
                None => dict_get(dict, "Parent").and_then(PdfObject::as_ref).map(|owner| Stage::Ancestor { owner, depth: depth + 1 }).or_else(|| no_binding.then_some(Stage::RoleStart)),""")
    text = replace(text, """        let program = self.program.as_mut().expect("active PDF page program");
        let turn = if let Some((_, index)) = self.page_font {""", """        let program = self.program.as_mut().expect("active PDF page program");
        if let Some(PdfOp::PaintXObject { name }) = program.pending_operator() {
            if self.images.page_name(self.snapshot.as_ref().expect("active PDF image snapshot")) != Some(name.as_str()) {
                return self.defer(Barrier::Program);
            }
        }
        let turn = if let Some((_, index)) = self.page_font {""")
    text = replace(text, "fn content_turn(turn: PageContentReceipt)", """fn image_turn(turn: CosReceipt) -> ControlledLiftReceipt {
    let event = turn.status.map(|status| match status {
        CosStatus::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        CosStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        _ => Event::More,
    });
    ControlledLiftReceipt { event, work: turn.work, allocated_bytes: turn.allocated_bytes, released_bytes: turn.released_bytes }
}

fn content_turn(turn: PageContentReceipt)""")
    return text


def program(text):
    return replace(text, "    /// 🫴️ Transfers the native page content vector", """    /// 👀️ Borrows a ready operator before the collector accepts its page resource binding.
    pub fn pending_operator(&self) -> Option<&PdfOp> { self.source.peek_operator() }

    /// 🫴️ Transfers the native page content vector""")


def importer(text):
    return text.replace("inline_annotation", "inline_item").replace("annotation_body", "inline_body").replace("reserving_empty_objects()", "reserving_empty_backing()")


def stage(base, output):
    for name, path, update in [("document", DOCUMENT, document), ("program", PROGRAM, program), ("import", IMPORT, importer)]:
        original = (base / path).read_text()
        patched = update(original)
        (output / f"🎞️{name}-integration.rs").write_text(patched)
        print(f"[DEBUG] {name}: {len(patched.splitlines()) - len(original.splitlines()):+d} source lines")


if __name__ == "__main__":
    stage(Path(sys.argv[1]), Path(__file__).resolve().parent / "🗑️generated")
