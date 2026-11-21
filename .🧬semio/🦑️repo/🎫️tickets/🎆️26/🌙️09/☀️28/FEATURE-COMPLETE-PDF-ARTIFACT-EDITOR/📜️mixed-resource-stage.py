"""🔂 Stages bounded text and image resource admission and retains both bindings."""

from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
stage = ticket / "🗑️generated/mixed"
stage.mkdir(exist_ok=True)
base = Path(sys.argv[1]).resolve()
lift = base / "🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs"
image_path = base / "🔨️modules/🖼️images/🎞️controlled/🦀️.rs"

def replace(source, old, new):
    assert source.count(old) == 1, old
    return source.replace(old, new)

image = image_path.read_text()
first = image.index("    /// 📃️ Reads a single indirect XObject")
end = image.index("        let PdfObject::Dict(entries)", first)
image = image[:first] + '''    /// 📃️ Reads the one indirect image binding in a qualified XObject subdictionary.
    pub fn from_xobjects(value: &PdfObject) -> Option<Self> {
''' + image[end:]
image = replace(image, "let PdfObject::Dict(entries) = &resources.value", "let PdfObject::Dict(entries) = value")

source = lift.read_text()
source = replace(source, '''    BranchResources(PageAttribute), ResourceOwner { owner: ObjRef, resume: Resume },
    Image(Resume),
    Font { resources: ObjRef, owner: ObjRef, resume: Resume },''', '''    Resources { location: ResourceLocation, resume: Resume },
    Image { resume: Resume, font: Option<(ResourceLocation, ObjRef)> },
    Font { resources: ResourceLocation, owner: ObjRef, resume: Resume },''')
source = replace(source, '''enum Resume { Walk, Inherit, Role }
''', '''enum Resume { Walk, Inherit, Role }
#[derive(Clone, Copy, PartialEq, Eq)]
enum ResourceLocation { Direct(PageAttribute), Indirect(ObjRef) }

impl ResourceLocation {
    fn inspect(self, source: &mut dyn ObjectSource, read: &mut dyn FnMut(Option<&PdfObject>)) {
        match self {
            Self::Direct(attribute) => inspect_attribute(source, attribute, read),
            Self::Indirect(owner) => source.inspect(owner, read),
        }
    }
}
''')
source = replace(source, "page_font: Option<(ObjRef, usize)>", "page_font: Option<(ResourceLocation, usize)>")
source = source.replace("Stage::Image(_)", "Stage::Image { .. }")
source = replace(source, '''            Stage::BranchResources(attribute) => self.resource_attribute(source, attribute, Resume::Walk),
            Stage::ResourceOwner { owner, resume } => self.resource_owner(source, owner, resume),
            Stage::Image(resume) => self.image(source, grant, resume),''', '''            Stage::Resources { location, resume } => self.resources(source, location, resume),
            Stage::Image { resume, font } => self.image(source, grant, resume, font),''')
source = replace(source, "self.stage = Stage::BranchResources(attribute)", "self.stage = Stage::Resources { location: ResourceLocation::Direct(attribute), resume: Resume::Walk }")
source = replace(source, "self.resource_attribute(source, attribute, Resume::Inherit)", "self.resources(source, ResourceLocation::Direct(attribute), Resume::Inherit)")
first = source.index("    fn resource_attribute(")
end = source.index("        let mut name = ", first)
source = source[:first] + '''    fn resources(&mut self, source: &mut dyn ObjectSource, location: ResourceLocation, resume: Resume) -> ControlledLiftReceipt {
        let mut owner = None;
        let mut bindings = None;
        location.inspect(source, &mut |value| {
            owner = value.and_then(PdfObject::as_ref);
            bindings = resource_bindings(value);
        });
        if let (ResourceLocation::Direct(_), Some(owner)) = (location, owner) {
            self.stage = Stage::Resources { location: ResourceLocation::Indirect(owner), resume };
            return work(0, 0);
        }
        let Some((font, image)) = bindings else { return self.defer(Barrier::Resources); };
        if resume == Resume::Role {
            let snapshot = self.snapshot.as_ref().expect("active PDF resource snapshot");
            let same_font = self.page_font.map(|(resources, index)| (resources, self.font_owners[index])) == font.map(|owner| (location, owner));
            let same_image = image.map_or_else(|| self.images.page_name(snapshot).is_none(), |binding| self.images.matches_page(snapshot, &binding));
            if !same_font || !same_image { return self.defer(Barrier::Resources); }
            self.resume(resume);
        } else {
            if font.is_some() && resume == Resume::Walk && !self.snapshot_mut().pages.is_empty() { return self.defer(Barrier::Resources); }
            if let Some(binding) = image {
                self.images.begin(binding);
                self.stage = Stage::Image { resume, font: font.map(|owner| (location, owner)) };
            } else {
                self.resume_resources(resume, font.map(|owner| (location, owner)));
            }
        }
        work(0, 0)
    }

    fn resume_resources(&mut self, resume: Resume, font: Option<(ResourceLocation, ObjRef)>) {
        if let Some((resources, owner)) = font { self.stage = Stage::Font { resources, owner, resume }; }
        else { self.resume(resume); }
    }

    fn image(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, resume: Resume, font: Option<(ResourceLocation, ObjRef)>) -> ControlledLiftReceipt {
        let Some(turn) = self.images.advance(source, self.snapshot.as_mut().expect("active PDF image snapshot"), grant) else { return self.defer(Barrier::Resources); };
        if let Ok(CosStatus::Complete { end }) = turn.status {
            if resume == Resume::Inherit { self.images.set_page(Some(end)); }
            self.resume_resources(resume, font);
        }
        image_turn(turn)
    }

    fn font(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, resources: ResourceLocation, owner: ObjRef, resume: Resume) -> ControlledLiftReceipt {
''' + source[end:]
source = replace(source, '''        source.inspect(resources, &mut |value| {
            let Some(entry) = font_entry(value) else { return; };''', '''        resources.inspect(source, &mut |value| {
            let Some(entry) = value.and_then(|value| value.dict_get("Font")).and_then(single_resource) else { return; };''')
first = source.index('            step = match dict_get(dict, "Resources")')
end = source.index("        });", first)
source = source[:first] + '''            step = if let Some(entry) = dict.iter().position(|entry| entry.key == "Resources") {
                Some(Stage::Resources { location: ResourceLocation::Direct(PageAttribute { owner, entry }), resume: Resume::Role })
            } else {
                dict_get(dict, "Parent").and_then(PdfObject::as_ref).map(|owner| Stage::Ancestor { owner, depth: depth + 1 }).or_else(|| no_binding.then_some(Stage::RoleStart))
            };
''' + source[end:]
source = replace(source, '''                + context.map_or(0, |_| size_of::<PdfGraphPath>() + "Font".len());''', '''                + context.map_or(0, |(location, _)| size_of::<PdfGraphPath>() + "Font".len()
                    + if matches!(location, ResourceLocation::Direct(_)) { size_of::<PdfGraphPath>() + "Resources".len() } else { 0 });''')
source = replace(source, '''                dependencies.push(PdfGraphIdentity { owner: resources, path: vec![PdfGraphPath::Entry { key: "Font".into() }] });''', '''                let font = PdfGraphPath::Entry { key: "Font".into() };
                let identity = match resources {
                    ResourceLocation::Indirect(owner) => PdfGraphIdentity { owner, path: vec![font] },
                    ResourceLocation::Direct(attribute) => PdfGraphIdentity { owner: attribute.owner, path: vec![PdfGraphPath::Entry { key: "Resources".into() }, font] },
                };
                dependencies.push(identity);''')
first = source.index("fn font_entry(")
end = source.index("fn standard_type1(", first)
source = source[:first] + '''fn resource_bindings(value: Option<&PdfObject>) -> Option<(Option<ObjRef>, Option<ControlledImageBinding>)> {
    if empty_dictionary(value) { return Some((None, None)); }
    let dict = value?.as_dict().filter(|dict| dict.len() <= 2)?;
    let mut font = None;
    let mut image = None;
    for entry in dict {
        match entry.key.as_str() {
            "Font" if font.is_none() => font = Some(single_resource(&entry.value)?.value.as_ref()?),
            "XObject" if image.is_none() => image = Some(ControlledImageBinding::from_xobjects(&entry.value)?),
            _ => return None,
        }
    }
    Some((font, image))
}

fn single_resource(value: &PdfObject) -> Option<&PdfDictEntry> {
    let [entry] = value.as_dict()? else { return None; };
    Some(entry)
}

''' + source[end:]
(stage / "images.rs").write_text(image)
(stage / "document.rs").write_text(source)
