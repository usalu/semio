"""Slice 4: the controlled lifter types any number of image resources per document and per page (growable quoted owner and page tables instead of 32 fixed slots), released in bounded units with the rest of the lift state."""
import sys
from pathlib import Path

base = Path(__file__).resolve().parents[7] / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"


def patch(relative, pairs):
    path = base / relative
    text = path.read_text(encoding="utf-8")
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit(f"{relative}: anchor found {text.count(old)} times:\n{old[:200]}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")


patch("🔨️modules/🖼️images/🎞️controlled/🦀️.rs", [
    ("""/// 🪟️ Coordinates page bindings and the independently owned image roles of one native lift.
#[derive(Default)]
pub struct ControlledImageResources {
    owners: [ObjRef; Self::MAX_COUNT],
    active: Option<ImageAdmission>,
    page_images: [u8; Self::MAX_COUNT],
    page_length: usize,
}

impl ControlledImageResources {
    /// 📏️ Limits the number of distinct image resources on the controlled path.
    pub const MAX_COUNT: usize = 32;

    /// 🪪️ The native owner of the image typed in `slot`.
    pub fn owner(&self, slot: usize) -> ObjRef { self.owners[slot] }

    /// 📑️ Starts a new page with no bound image resources.
    pub fn clear_page(&mut self) { self.page_length = 0; }

    /// 📌️ Binds one admitted image slot to the active page; false when the page exceeds the slot table.
    pub fn push_page(&mut self, slot: usize) -> bool {
        let Some(entry) = self.page_images.get_mut(self.page_length) else { return false; };
        let Ok(slot) = u8::try_from(slot) else { return false; };
        *entry = slot;
        self.page_length += 1;
        true
    }

    /// 🔗️ Reports whether the active page binds an image whose typed ID is the authored XObject name.
    pub fn page_contains(&self, snapshot: &PdfSnapshot, name: &str) -> bool {
        self.page_images[..self.page_length].iter().any(|slot| snapshot.images[usize::from(*slot)].id == name)
    }""",
     """/// 🪟️ Coordinates page bindings and the independently owned image roles of one native lift: the owner of every typed image (in image order) and the images the active page binds grow under quotes, so a document or page may bind any number of images.
#[derive(Default)]
pub struct ControlledImageResources {
    owners: Vec<ObjRef>,
    active: Option<ImageAdmission>,
    page_images: Vec<u32>,
}

impl ControlledImageResources {
    /// 🪪️ The native owner of the image typed in `slot`.
    pub fn owner(&self, slot: usize) -> ObjRef { self.owners[slot] }

    /// 📑️ Starts a new page with no bound image resources; the page table keeps its capacity.
    pub fn clear_page(&mut self) { self.page_images.clear(); }

    /// 📏️ The growth the page table needs before it can bind one more image (zero when it has room).
    pub fn page_growth(&self) -> usize { table_growth(&self.page_images, 1) }

    /// 📥️ Grows the page table exactly as [`Self::page_growth`] quoted; false when the allocator refuses.
    pub fn grow_page(&mut self) -> bool { grow_table(&mut self.page_images, 1) }

    /// 📌️ Binds one admitted image slot to the active page; false when the table has no room left ([`Self::grow_page`] first).
    pub fn push_page(&mut self, slot: usize) -> bool {
        let Ok(slot) = u32::try_from(slot) else { return false; };
        if self.page_images.len() == self.page_images.capacity() { return false; }
        self.page_images.push(slot);
        true
    }

    /// 🔗️ Reports whether the active page binds an image whose typed ID is the authored XObject name.
    pub fn page_contains(&self, snapshot: &PdfSnapshot, name: &str) -> bool {
        self.page_images.iter().any(|slot| snapshot.images[*slot as usize].id == name)
    }

    /// 📦️ The bytes the owner and page tables hold.
    pub fn held_bytes(&self) -> usize { self.owners.capacity() * size_of::<ObjRef>() + self.page_images.capacity() * size_of::<u32>() }

    /// 🧺️ Drops both tables once the lift credited [`Self::held_bytes`].
    pub fn release(&mut self) {
        self.owners = Vec::new();
        self.page_images = Vec::new();
    }"""),
    ("""            let bytes = 2 * binding.length + 2 * size_of::<PdfGraphIdentity>() + samples.reference_bytes()
                + table_growth(&snapshot.images, 1) + table_growth(&snapshot.admitted_stream_roles, 1);
            if bytes > grant.allocation_bytes { return Some(sample_receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0)); }
            if !grow_table(&mut snapshot.images, 1) || !grow_table(&mut snapshot.admitted_stream_roles, 1) {""",
     """            let bytes = 2 * binding.length + 2 * size_of::<PdfGraphIdentity>() + samples.reference_bytes()
                + table_growth(&snapshot.images, 1) + table_growth(&snapshot.admitted_stream_roles, 1) + table_growth(&self.owners, 1);
            if bytes > grant.allocation_bytes { return Some(sample_receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0)); }
            if !grow_table(&mut snapshot.images, 1) || !grow_table(&mut snapshot.admitted_stream_roles, 1) || !grow_table(&mut self.owners, 1) {"""),
    ("""            snapshot.images.push(image);
            self.owners[count] = binding.owner;""",
     """            snapshot.images.push(image);
            self.owners.push(binding.owner);"""),
    ("""        if count == Self::MAX_COUNT || snapshot.images.iter().any(|image| image.id == binding.name()) { return None; }""",
     """        if self.owners.len() != count || snapshot.images.iter().any(|image| image.id == binding.name()) { return None; }"""),
])

patch("🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs", [
    ("""    fn image(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, index: usize, resume: Resume) -> ControlledLiftReceipt {
        let Some(turn)""",
     """    fn image(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, index: usize, resume: Resume) -> ControlledLiftReceipt {
        let growth = self.images.page_growth();
        if resume == Resume::Inherit && growth != 0 {
            if let Some(denied) = budget(grant, growth, 0) { return denied; }
            if !self.images.grow_page() { return ControlledLiftReceipt { event: Err("unable to grow the PDF page image table"), ..work(0, 0) }; }
            return work(growth, 0);
        }
        let Some(turn)"""),
    ("""                let held = self.navigation_bytes() + self.cold_bytes + self.states.held_bytes() + self.mirror_bytes() + if fonts { self.font_resources.held_bytes() } else { 0 };""",
     """                let held = self.navigation_bytes() + self.cold_bytes + self.states.held_bytes() + self.mirror_bytes() + self.images.held_bytes() + if fonts { self.font_resources.held_bytes() } else { 0 };"""),
    ("""        } else if self.states.held_bytes() != 0 {
            self.states = GraphicsStates::default();""",
     """        } else if self.states.held_bytes() != 0 {
            self.states = GraphicsStates::default();
        } else if self.images.held_bytes() != 0 {
            self.images.release();"""),
    ("""        self.states = GraphicsStates::default();
        if fonts { self.font_resources.release(); }""",
     """        self.states = GraphicsStates::default();
        self.images.release();
        if fonts { self.font_resources.release(); }"""),
])

patch("🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🧪️tests/🔬️unit/🦀️.rs", [
    ("""fn typed(mut snapshot: PdfSnapshot) -> PdfSnapshot {""",
     """/// 🖼️ LAW: a page painting more images than the former 32-slot table lifts on the controlled path exactly like the reference lifter, every unit inside its grant.
#[test]
fn a_page_with_forty_images_lifts_like_the_reference_lifter() {
    use crate::standards::v1_7::subsets::base::{io::export::encode_pdf, schema::snapshot::{PdfColorSpace, PdfImage, PdfOp, PdfPage}};
    let mut page = PdfPage::new(400.0, 400.0);
    let mut typed = PdfSnapshot::default();
    for index in 0..40u8 {
        let id = format!("I{index}");
        typed.images.push(PdfImage::samples(id.clone(), 2, 2, PdfColorSpace::DeviceGray, 8, vec![u32::from(index); 4]));
        page.content.push(PdfOp::PaintXObject { name: id });
    }
    typed.pages.push(page);
    let retained = decode_pdf(&encode_pdf(&typed).unwrap()).unwrap();
    let reference = typed(lift_document(&retained.trailer, &retained.declared_version, &mut GraphSource::new(&retained.objects)).unwrap());
    let controlled = typed(controlled_lift(&retained).unwrap_or_else(|barrier| panic!("forty images stop at {barrier}")));
    assert_eq!(controlled.images.len(), 40);
    assert_eq!(controlled, reference);
}

fn typed(mut snapshot: PdfSnapshot) -> PdfSnapshot {"""),
])
print("applied slice 4 growable image tables")
