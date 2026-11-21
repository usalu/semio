/** 📐 Stages the semantic Font size edit, shared editor declarations and its mounted native law. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFile, writeFile, mkdir} from "node:fs/promises";
import {join, resolve} from "node:path";

const top = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets";
const base = join(top, "🧱️base");
const page = join(base, "✏️editor/🖼️page");
const stage = join(import.meta.dir, "🗑️generated/font-size");
const hash = (text: string) => createHash("sha256").update(text).digest("hex");
if (process.argv[2] === "--apply") {
    const rows = JSON.parse(await readFile(join(stage, "overlay.json"), "utf8")) as [string, string, string][];
    const verify = async (file: string, digest: string) =>
        assert.equal(hash(await readFile(file, "utf8")), digest, "Shared source changed since the verified stage: " + file);
    for (const [file, digest] of rows) await verify(file, digest);
    for (const [file, digest, after] of rows) {
        await verify(file, digest);
        await writeFile(file, after);
    }
    console.log("[DEBUG] Applied " + rows.length + " verified font-size changes");
    process.exit(0);
}
const files = new Map<string, {before: string; after: string}>();
const check = async (file: string, anchor: string, replacement: string, count = 1) => {
    const row = files.get(file) ?? await readFile(file, "utf8").then(before => ({before, after: before}));
    assert.equal(row.after.split(anchor).length - 1, count, file + " " + anchor);
    row.after = row.after.replaceAll(anchor, () => replacement);
    files.set(file, row);
};

const match = (file: string, before: string, after: string) => check(join(page, file), before, after);
await match("🧱️model/🦀️.rs", 'const PAGE_ACTIONS: [&str; 67] = [', 'const PAGE_ACTIONS: [&str; 68] = [');
await match("🧱️model/🦀️.rs", '    "set-font",', '    "set-font",\n    "set-font-size",');
await match("🔎️inspector/🦀️.rs", 'geometry("pdf-object-size", "Font size", "Schriftgröße", "height", object.font_size, object.x, object.y, object.width, object.font_size)', 'placed("pdf-object-size", "Font size", "Schriftgröße", "set-font-size", "size", object.font_size, &[])');
await match("🪟️windows/🦀️.rs", '            action("set-font", "Set Font", "Schrift setzen", vec![page(), object(), text()]),', '            action("set-font", "Set Font", "Schrift setzen", vec![page(), object(), text()]),\n            action("set-font-size", "Set Font Size", "Schriftgröße setzen", vec![page(), object(), ActionArgDef::number("size", LocalizedLabel::native("Font size", "Schriftgröße")).required()]),');
await match("🎬️actions/🦀️.rs", `    if action == "move-page" {
        return Ok(payload_edit(action, page_index()?, String::new(), String::new(), String::new(), req_num(args, "x")?, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
    }
    let (page, object, text, extra, x, y, width, height, red, green, blue) = match action {`,
`    let (page, object, text, extra, x, y, width, height, red, green, blue) = match action {
        "move-page" => (page_index()?, String::new(), String::new(), String::new(), req_num(args, "x")?, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-font-size" => (page_index()?, object_id()?, String::new(), String::new(), 0.0, 0.0, 0.0, req_num(args, "size")?, 0.0, 0.0, 0.0),`);
await match("🧮️content-ops/🦀️.rs", 'pub(super) fn delete_text(content: &mut Vec<PdfOp>, object: &PageObject) -> Result<(), Fault> {',
`pub(super) fn restyle_text(content: &mut Vec<PdfOp>, object: &PageObject, paint: PdfOp, name: String, size: f64) -> Result<(), Fault> {
    let mut replacement = Vec::new();
    let paint = show_with_explicit_line(paint, &mut replacement);
    replacement.extend([PdfOp::SetFont { name, size }, paint]);
    if !object.font.is_empty() { replacement.push(PdfOp::SetFont { name: object.font.clone(), size: object.font_size }); }
    replacement.extend(text_cursor_after(content, object)?);
    content.splice(object.end..=object.end, replacement);
    Ok(())
}

pub(super) fn delete_text(content: &mut Vec<PdfOp>, object: &PageObject) -> Result<(), Fault> {`);
await match("🖱️object-edits/🦀️.rs", '        "delete" if object.kind == ObjectKind::Text => delete_text(&mut content, &object)?,',
`        "set-font-size" if object.kind == ObjectKind::Text => {
            if !height.is_finite() || height <= 0.0 || object.font.is_empty() { return Err(fault("a text run needs an active font and a positive finite size")); }
            if height == object.font_size { return Ok(Vec::new()); }
            let paint = content[object.end].clone();
            restyle_text(&mut content, &object, paint, object.font.clone(), height)?;
        }
        "delete" if object.kind == ObjectKind::Text => delete_text(&mut content, &object)?,`);
await match("🖱️object-edits/🦀️.rs", `    content[object.end] = target.rebind_show(&content[object.end], fonts.codec(&object.font))
        .ok_or_else(|| fault("the selected PDF font cannot preserve the shown text"))?;
    let restore = text_cursor_after(&content, &object)?;
    content.splice(object.end + 1..object.end + 1, restore);
    if !object.font.is_empty() {
        content.insert(object.end + 1, PdfOp::SetFont { name: object.font, size: object.font_size });
    }
    content.insert(object.end, PdfOp::SetFont { name: font.id.clone(), size: object.font_size });`,
`    let paint = target.rebind_show(&content[object.end], fonts.codec(&object.font))
        .ok_or_else(|| fault("the selected PDF font cannot preserve the shown text"))?;
    restyle_text(&mut content, &object, paint, font.id.clone(), object.font_size)?;`);

const advance = `fn append_advance(content: &mut Vec<PdfOp>, advance: f64, size: f64, scale: f64) -> Result<(), Fault> {
    if advance != 0.0 {
        if size * scale == 0.0 { return Err(fault("cannot restore an advance with zero text scale")); }
        let amount = -advance * 100_000.0 / (size * scale);
        content.push(PdfOp::ShowTextArray { items: vec![PdfTextArrayItem::Adjust { amount }] });
    }
    Ok(())
}`;
await match("🧮️content-ops/🦀️.rs", advance + "\n\n", "");
const outline = "pub fn set_outline(";
await check(join(base, "✏️editor/📄️document/🦀️.rs"), outline,
    "/// 📝️ Appends a position-only PDF text adjustment in original font units.\npub " + advance + "\n\n" + outline);

for (const name of ["🗄️a", "♿️ua", "🧾️vt", "🧱️base", "🖨️x", "⚕️h", "📐️e"]) {
    await check(join(top, name, "✏️editor/🦀️.rs"), '"set-font", "set-outline"', '"set-font", "set-font-size", "set-outline"', 2);
}

const fixture = join(base, "🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json");
await check(fixture, '  "fontReassignment": {', `  "fontSize": {
    "before": 20,
    "after": 16,
    "sameLine": "Same",
    "sameLineCode": "53616D65",
    "followingLine": "Tail"
  },
  "fontReassignment": {`);
const tests = join(base, "✏️editor/🧪️tests/🔬️unit/🦀️.rs");
await check(tests, '    let case = &fixture["fontReassignment"];', '    let case = &fixture["fontReassignment"];\n    let size = &fixture["fontSize"];');
await check(tests, 'BT /Fsrc 20 Tf 10 50 Td 24 TL [<{byte:02X}> {adjust} <{byte:02X}>] TJ T* (Tail) Tj ET',
    'BT /Fsrc {} Tf 10 50 Td 24 TL [<{byte:02X}> {adjust} <{byte:02X}>] TJ [<{}> -20] TJ T* ({}) Tj ET", size["before"], size["sameLineCode"].as_str().unwrap(), size["followingLine"].as_str().unwrap() + "');
await check(tests, '.as_str().unwrap() + "");', '.as_str().unwrap());');
await check(tests, `    assert_eq!(before[1].text.as_str(), case["secondLine"].as_str().unwrap());
    let args = DslValue::from(json!({"page": 0, "object": before[0].id, "text": case["targetFont"]}));
    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, "set-font", Some(args.clone()), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap();`,
`    assert_eq!(before[1].text.as_str(), size["sameLine"].as_str().unwrap());
    assert_eq!(before[2].text.as_str(), size["followingLine"].as_str().unwrap());
    let first = before[0].id.as_str();
    let original_same_line = app.snapshot().unwrap().pages[0].content[before[1].end].clone();
    for (action, args) in [
        ("set-font", json!({"page": 0, "object": first, "text": case["targetFont"]})),
        ("set-font-size", json!({"page": 0, "object": first, "field": "size", "value": size["after"]})),
    ] {
        semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
            &mut app, action, Some(DslValue::from(args.clone())), &meta, &mut identity, scope.cx(), policy.close
        ).await).unwrap();
        semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
            &mut app, meta.instance_id, policy, &mut identity, scope.cx()
        ).await).unwrap();
    }`);
await check(tests, `        assert_eq!(after[1].text, before[1].text);
        assert_eq!((after[1].x, after[1].y), (before[1].x, before[1].y));
        for (object, expected) in after.iter().zip(["targetFont", "sourceFont"]) {`,
`        assert_eq!(after[0].font_size, size["after"].as_f64().unwrap());
        assert_eq!(after.len(), before.len());
        assert_eq!(snapshot.pages[0].content[after[1].end], original_same_line);
        for (after, before) in after[1..].iter().zip(&before[1..]) {
            assert_eq!((&after.text, after.font_size), (&before.text, before.font_size));
            assert!((after.x - before.x).abs() < 1e-5 && (after.y - before.y).abs() < 1e-5);
        }
        for (object, expected) in after.iter().zip(["targetFont", "sourceFont", "sourceFont"]) {`);

const {measureRustCrate} = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const overlay = new Map([...files].map(([file, {after}]) => [resolve(file), after]));
await mkdir(stage, {recursive: true});
await writeFile(join(stage, "overlay.json"), JSON.stringify([...files].map(([file, row]) => [file, hash(row.before), row.after])));
await writeFile(join(stage, "font-size.json"), JSON.stringify(JSON.parse(files.get(fixture)!.after).fontSize));
console.log("[DEBUG] Staged " + files.size + " font-size paths");

for (const [name, unit] of [
    ["page", page],
    ["page-tests", join(page, "🧪️tests/🔬️unit")],
    ["controller", join(top, "🧾️vt/🧬️schema")],
    ["base", base],
    ["mounted-tests", join(base, "🧬️schema/📸️snapshot/🧪️tests/🔬️pdf-snapshot-value")],
    ["page-tests-mirror", join(top, "../../4️⃣1.4/🪆️subsets/🗄️a/🧬️schema")],
    ["document", join(base, "✏️editor/📄️document")],
]) {
    const directory = resolve(unit, "📦️packages/🦀️rust");
    const test = true;
    const before = await measureRustCrate(process.cwd(), join(directory, "🦀️.rs"), test, new Map(), directory, []);
    const after = await measureRustCrate(process.cwd(), join(directory, "🦀️.rs"), test, overlay, directory, []);
    console.log("[DEBUG] " + name + " " + before.lines + " -> " + after.lines + " active test lines, " + after.files + " files, " + after.bytes + " bytes");
    assert(after.lines <= 4000 && after.files <= 32 && after.bytes <= 262144, name + " exceeds its source boundary");
}
