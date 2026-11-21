"""🧷 Stage bounded Identity-H TrueType admission against the exact existing font sources."""

import hashlib
import json
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "font-cid-truetype"
FONTS = "🔨️modules/🔤️fonts"
OWNER = FONTS + "/🎚️controlled-opentype/🦀️.rs"
LIFT = "🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs"


def once(text, before, after):
    """🎯 Change one exact source anchor."""
    assert text.count(before) == 1, before
    return text.replace(before, after, 1)


def stage(base):
    """📦 Write the candidate maps without modifying the worktree."""
    original = (base / OWNER).read_text(encoding="utf8")
    edits = [
        ("//! 🔤️ Admits one borrowed simple OpenType program and its direct Unicode maps.", "//! 🔤️ Admits one borrowed embedded font program and its direct Unicode maps."),
        ("ObjRef, PdfBaseEncoding, PdfDictEntry", "ObjRef, PdfBaseEncoding, PdfCMap, PdfCidFont, PdfCidToGid, PdfCidWidthRun, PdfDictEntry"),
        ('const KIND: &str = "s.stdio.font.opentype";', 'const OPENTYPE: &str = "s.stdio.font.opentype";\nconst TRUETYPE: &str = "s.stdio.font.truetype";'),
        ("enum Phase { Descriptor", "enum Phase { Descendant, Descriptor"),
        ("/// 🧾️ Transfers a simple font", "/// 🧾️ Transfers an embedded font"),
        ("    pub dependencies: [ObjRef; 3],", "    dependencies: FontDependencies,"),
        ("/// 🔏️ Retains only bounded", """impl AdmittedSimpleOpenType {
    /// 🔗 Borrows every graph owner that the typed font depended on.
    pub fn dependencies(&self) -> &[ObjRef] { self.dependencies.owners() }
}

/// 🔏️ Retains only bounded"""),
        ("    descriptor_owner: ObjRef,\n", "    descendant: Option<Composite>,\n    descriptor_owner: Option<ObjRef>,\n"),
        ("    /// 🔍️ Qualifies ordinary WinAnsi OpenType font dictionaries without copying their arrays.", "    /// 🔍️ Qualifies a horizontal font dictionary without copying its arrays."),
        ('''        let dict = qualified(source, &["Type", "Subtype", "BaseFont", "Encoding", "FirstChar", "LastChar", "Widths", "FontDescriptor", "ToUnicode"])?;
        if name(dict, "Type")? != "Font" || name(dict, "Subtype")? != "Type1" || name(dict, "Encoding")? != "WinAnsiEncoding" { return None; }
        let first = u8::try_from(integer(dict, "FirstChar")?).ok()?;
        let last = u8::try_from(integer(dict, "LastChar")?).ok()?;
        let count = usize::from(last.checked_sub(first)?) + 1;
        if source.dict_get("Widths")?.as_array()?.len() != count { return None; }
        let mut hash = Sha256::new();
        hash.update(KIND.as_bytes());''', '''        let (dict, descendant, descriptor_owner, first, count) = match source.dict_get("Subtype")?.as_name()? {
            "Type1" => {
                let dict = qualified(source, &["Type", "Subtype", "BaseFont", "Encoding", "FirstChar", "LastChar", "Widths", "FontDescriptor", "ToUnicode"])?;
                if name(dict, "Encoding")? != "WinAnsiEncoding" { return None; }
                let first = u8::try_from(integer(dict, "FirstChar")?).ok()?;
                let last = u8::try_from(integer(dict, "LastChar")?).ok()?;
                let count = usize::from(last.checked_sub(first)?) + 1;
                if source.dict_get("Widths")?.as_array()?.len() != count { return None; }
                (dict, None, Some(reference(dict, "FontDescriptor")?), u32::from(first), count)
            }
            "Type0" => {
                let dict = qualified(source, &["Type", "Subtype", "BaseFont", "Encoding", "DescendantFonts", "ToUnicode"])?;
                if name(dict, "Encoding")? != "Identity-H" { return None; }
                let [child] = get(dict, "DescendantFonts")?.as_array()? else { return None; };
                (dict, Some(Composite { owner: child.as_ref()?, default_width: 1000., identity: false }), None, 0, 0)
            }
            _ => return None,
        };
        if name(dict, "Type")? != "Font" { return None; }
        let mut hash = Sha256::new();
        hash.update(if descendant.is_some() { TRUETYPE } else { OPENTYPE }.as_bytes());'''),
        ('''            owner, descriptor_owner: reference(dict, "FontDescriptor")?, unicode_owner: reference(dict, "ToUnicode")?,
            font: ShortName::new(name(dict, "BaseFont")?)?, first: u32::from(first), count, descriptor: Default::default(),''', '''            owner, descendant, descriptor_owner, unicode_owner: reference(dict, "ToUnicode")?,
            font: ShortName::new(name(dict, "BaseFont")?)?, first, count, descriptor: Default::default(),'''),
        ("phase: Phase::Descriptor, deferred", "phase: if descendant.is_some() { Phase::Descendant } else { Phase::Descriptor }, deferred"),
        ("size_of::<PdfFont>() + 2 * size_of::<PdfAdmittedStreamRole>()", "size_of::<PdfFont>() + 2 * size_of::<PdfAdmittedStreamRole>() + size_of::<FontDependencies>()"),
        ("        match self.phase {\n            Phase::Descriptor => {", '''        match self.phase {
            Phase::Descendant => {
                let child = self.descendant.as_mut().expect("pending CIDFont descendant");
                let mut shape = None;
                inspect(child.owner, &mut |value| shape = value.and_then(|value| cid_font(value, self.font.text())));
                let Some((descriptor, default_width, identity, first, count)) = shape else { return self.defer(); };
                child.default_width = default_width;
                child.identity = identity;
                self.descriptor_owner = Some(descriptor);
                self.first = first;
                self.count = count;
                self.phase = Phase::Descriptor;
                more()
            }
            Phase::Descriptor => {'''),
        ("                inspect(self.descriptor_owner, &mut |value| {\n                    shape = value.and_then(|value| descriptor(value, self.font.text()));", "                let program_key = if self.descendant.is_some() { \"FontFile2\" } else { \"FontFile3\" };\n                inspect(self.descriptor_owner.expect(\"qualified descriptor owner\"), &mut |value| {\n                    shape = value.and_then(|value| descriptor(value, self.font.text(), program_key));"),
        ("                self.phase = Phase::Widths;", "                self.phase = if self.count == 0 { Phase::Program } else { Phase::Widths };"),
        ('''                inspect(self.owner, &mut |value| {
                    next = value.and_then(|font| font.dict_get("Widths")?.as_array()?.get(self.widths.len())?.as_i64());
                });
                let Some(width) = next else { return self.defer(); };
                self.widths.push(width as f64);''', '''                if let Some(child) = self.descendant {
                    inspect(child.owner, &mut |value| {
                        next = value.and_then(|font| font.dict_get("W")?.as_array()?.get(1)?.as_array()?.get(self.widths.len())?.as_f64())
                            .filter(|value| value.is_finite() && *value >= 0.);
                    });
                } else {
                    inspect(self.owner, &mut |value| {
                        next = value.and_then(|font| font.dict_get("Widths")?.as_array()?.get(self.widths.len())?.as_i64()).map(|value| value as f64);
                    });
                }
                let Some(width) = next else { return self.defer(); };
                self.widths.push(width);'''),
        ('''                    if dict.len() != 1 || name(dict, "Subtype") != Some("OpenType") || !data.starts_with(b"OTTO") { return; }''', '''                    if dict.len() != 1 { return; }
                    let valid = if self.descendant.is_some() {
                        data.starts_with(&[0, 1, 0, 0]) && integer(dict, "Length1") == Some(data.len() as i64)
                    } else {
                        name(dict, "Subtype") == Some("OpenType") && data.starts_with(b"OTTO")
                    };
                    if !valid { return; }'''),
        ("                    if width != 1 { return self.defer(); }", "                    let code_width = if self.descendant.is_some() { 2 } else { 1 };\n                    if width != code_width { return self.defer(); }"),
        ("simple OpenType resource needs reference admission", "embedded font resource needs reference admission"),
        ('''        name.len() + 2 * self.font.length + 2 * artifact_reference_bytes(KIND) + 2 * size_of::<PdfGraphIdentity>()''', '''        let shared = name.len() + 2 * self.font.length + 2 * size_of::<PdfGraphIdentity>();
        let resource = if self.descendant.is_some() {
            self.font.length + "Identity-HAdobeIdentity".len() + 2 * artifact_reference_bytes(TRUETYPE)
                + if self.count == 0 { 0 } else { size_of::<PdfCidWidthRun>() }
        } else {
            2 * artifact_reference_bytes(OPENTYPE)
        };
        shared + resource'''),
        ('''        let resource = PdfFontProgram::OpenType { reference: artifact_reference(KIND, digest.finalize()) };
        let [primary, role] = std::mem::take(&mut self.maps);
        self.descriptor.font_name = self.font.text().to_owned();
        let font = PdfFont {
            id: name.to_owned(),
            kind: PdfFontKind::Type1 {
                base_font: self.font.text().to_owned(),
                encoding: PdfSimpleEncoding { base: Some(PdfBaseEncoding::WinAnsi), differences: Vec::new() },
                first_char: self.first, widths: std::mem::take(&mut self.widths),
                descriptor: Some(std::mem::take(&mut self.descriptor)), program: Some(resource.clone()),
            },
            to_unicode: Some(primary.expect("primary Unicode result")),
            extra: Vec::new(),
        };''', '''        let digest = digest.finalize();
        let [primary, role] = std::mem::take(&mut self.maps);
        let base_font = self.font.text();
        self.descriptor.font_name = base_font.to_owned();
        let descriptor = std::mem::take(&mut self.descriptor);
        let widths = std::mem::take(&mut self.widths);
        let (kind, resource) = if let Some(child) = self.descendant {
            let program = PdfFontProgram::TrueType { reference: artifact_reference(TRUETYPE, digest) };
            let descendant = PdfCidFont {
                true_type: true, base_font: base_font.to_owned(), system_info: Default::default(), descriptor,
                default_width: child.default_width,
                widths: if widths.is_empty() { Vec::new() } else { vec![PdfCidWidthRun { start_cid: self.first, widths }] },
                default_vertical: None, vertical_metrics: Vec::new(),
                cid_to_gid: child.identity.then_some(PdfCidToGid::Identity), program: Some(program.clone()), extra: Vec::new(),
            };
            (PdfFontKind::Type0 { base_font: base_font.to_owned(), cmap: PdfCMap::identity_h(), descendant }, program)
        } else {
            let program = PdfFontProgram::OpenType { reference: artifact_reference(OPENTYPE, digest) };
            (PdfFontKind::Type1 {
                base_font: base_font.to_owned(),
                encoding: PdfSimpleEncoding { base: Some(PdfBaseEncoding::WinAnsi), differences: Vec::new() },
                first_char: self.first, widths, descriptor: Some(descriptor), program: Some(program.clone()),
            }, program)
        };
        let font = PdfFont { id: name.to_owned(), kind, to_unicode: Some(primary.expect("primary Unicode result")), extra: Vec::new() };'''),
        ('''        Some(AdmittedSimpleOpenType { font, roles, dependencies: [self.descriptor_owner, self.unicode_owner, self.program_owner.unwrap()] })''', '''        let descriptor = self.descriptor_owner.expect("qualified descriptor owner");
        let child = self.descendant.map(|font| font.owner);
        let dependencies = FontDependencies {
            owners: [descriptor, self.unicode_owner, self.program_owner.unwrap(), child.unwrap_or(descriptor)],
            count: 3 + usize::from(child.is_some()),
        };
        Some(AdmittedSimpleOpenType { font, roles, dependencies })'''),
        ('''fn descriptor(value: &PdfObject, base_font: &str) -> Option<(PdfFontDescriptor, ObjRef)> {
    let dict = qualified(value, &["Type", "FontName", "Flags", "FontBBox", "ItalicAngle", "Ascent", "Descent", "CapHeight", "StemV", "FontFile3"])?;''', '''fn descriptor(value: &PdfObject, base_font: &str, program: &str) -> Option<(PdfFontDescriptor, ObjRef)> {
    let dict = qualified(value, &["Type", "FontName", "Flags", "FontBBox", "ItalicAngle", "Ascent", "Descent", "CapHeight", "StemV", program])?;'''),
        ("    if flags & 4 != 0 || flags & 32 == 0 { return None; }", "    if program == \"FontFile3\" && (flags & 4 != 0 || flags & 32 == 0) { return None; }"),
        ('}, reference(dict, "FontFile3")?))', '}, reference(dict, program)?))'),
        ("    embedded: Option<(usize, [ObjRef; 3])>,", "    embedded: Option<(usize, FontDependencies)>,"),
        ("|(_, owners)| owners.as_slice()", "|(_, dependencies)| dependencies.owners()"),
        ('''.expect("published OpenType resource")''', '''.expect("published embedded font resource")'''),
    ]
    updated = original
    for before, after in edits:
        if before == "simple OpenType resource needs reference admission":
            assert updated.count(before) == 2
            updated = updated.replace(before, after)
        else:
            updated = once(updated, before, after)
    helpers = (TICKET / "📜️controlled-cid-font-shape.rs").read_text(encoding="utf8")
    updated = once(updated, "fn filter_wire(", helpers + "\nfn filter_wire(")
    font_module = (base / (FONTS + "/🦀️.rs")).read_text(encoding="utf8")
    font_tests = (base / (FONTS + "/🧪️tests/🔬️unit/🦀️.rs")).read_text(encoding="utf8")
    lifter = (base / LIFT).read_text(encoding="utf8")
    binding = '''        (self.0.id == name && matches!(&self.0.kind, PdfFontKind::Type1 { program: Some(PdfFontProgram::OpenType { .. }), .. }))
            .then_some(PdfFixedCodeWidth::One)'''
    lifter = once(lifter.replace("Type1Binding", "PageFontBinding"), binding, '''        if self.0.id != name { return None; }
        match &self.0.kind {
            PdfFontKind::Type1 { program: Some(PdfFontProgram::OpenType { .. }), .. } => Some(PdfFixedCodeWidth::One),
            PdfFontKind::Type0 { descendant, .. } if descendant.true_type => Some(PdfFixedCodeWidth::Two),
            _ => None,
        }''')
    old_owner_test = (TICKET / "controlled-opentype-owner-test.rs").read_text(encoding="utf8")
    embedded_tests = (TICKET / "📜️controlled-font-owner-tests.rs").read_text(encoding="utf8")
    files = {
        OWNER: updated,
        FONTS + "/🦀️.rs": font_module,
        FONTS + "/🧪️tests/🔬️unit/🦀️.rs": once(font_tests, old_owner_test, embedded_tests),
        FONTS + "/🧪️tests/🔬️unit/📐️cid-truetype.txt": (OUT / "cid-truetype.txt").read_text(encoding="utf8"),
        LIFT: lifter,
    }
    OUT.mkdir(parents=True, exist_ok=True)
    roster = {}
    for index, (name, text) in enumerate(files.items()):
        updated = text.replace("AdmittedSimpleOpenType", "AdmittedEmbeddedFont").replace("PdfSimpleOpenTypeCursor", "PdfEmbeddedFontCursor")
        target = OUT / f"candidate-{index}.rs"
        path = base / name
        before = path.read_bytes() if path.is_file() else None
        after = updated.encode("utf8")
        target.write_bytes(after)
        roster[name] = {"before": hashlib.sha256(before).hexdigest() if before is not None else None, "after": hashlib.sha256(after).hexdigest(), "staged": str(target.resolve())}
    (OUT / "stage.json").write_text(json.dumps(roster, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Staged {len(roster)} qualified CID TrueType source paths")


def verify(base, apply=False):
    """🔒 Check every staged postimage and its exact live preimage."""
    roster = json.loads((OUT / "stage.json").read_text(encoding="utf8"))
    for name, receipt in roster.items():
        target = base / name
        staged = Path(receipt["staged"])
        assert len(str(target.resolve()).encode("utf-16-le")) // 2 <= 256, name
        current = hashlib.sha256(target.read_bytes()).hexdigest() if target.is_file() else None
        assert current == receipt["before"], name
        assert hashlib.sha256(staged.read_bytes()).hexdigest() == receipt["after"], name
    if apply:
        for name, receipt in roster.items():
            target = base / name
            before = hashlib.sha256(target.read_bytes()).hexdigest() if target.is_file() else None
            value = Path(receipt["staged"]).read_bytes()
            assert before == receipt["before"] and hashlib.sha256(value).hexdigest() == receipt["after"], name
            target.write_bytes(value)
        assert all(hashlib.sha256((base / name).read_bytes()).hexdigest() == receipt["after"] for name, receipt in roster.items())
    print(f"[DEBUG] {'Released' if apply else 'Verified'} {len(roster)} CID TrueType paths")


if __name__ == "__main__":
    root = Path(sys.argv[1])
    if sys.argv[2:] == ["--verify"]:
        verify(root)
    elif sys.argv[2:] == ["--apply-reviewed"]:
        verify(root, apply=True)
    elif not sys.argv[2:]:
        stage(root)
    else:
        raise ValueError("Expected no option or --verify")
