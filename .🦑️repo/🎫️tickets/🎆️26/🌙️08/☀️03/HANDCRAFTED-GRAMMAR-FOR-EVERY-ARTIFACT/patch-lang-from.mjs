import { readFileSync, writeFileSync, readdirSync, existsSync } from "fs";
import { join } from "path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const fw = readdirSync(repoRoot).find((x) => x.includes("framework"));
const derivePath = join(repoRoot, fw, "🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️lib.rs");
const schemaPath = join(repoRoot, fw, "🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/📦️lib.rs");
const schemaAlt = join(repoRoot, fw, "🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/⚡️implementations/🦀️rust/📦️lib.rs");
const schema = existsSync(schemaPath) ? schemaPath : schemaAlt;
console.log({ derivePath: existsSync(derivePath), schema: existsSync(schema), schema });

let derive = readFileSync(derivePath, "utf8");
if (!derive.includes("lang_from")) {
  derive = derive.replace(
    `    lang: Option<String>,
    /// \`#[dsl(coord)]\``,
    `    lang: Option<String>,
    /// \`#[dsl(lang_from = "language_id")]\` — fence language from a sibling Text field at print/parse time.
    lang_from: Option<String>,
    /// \`#[dsl(coord)]\``
  );
  // if that didn't match, try after lang block differently
  if (!derive.includes("lang_from")) {
    derive = derive.replace(
      `out.lang = Some(value.value());
            } else if meta.path.is_ident("coord") {`,
      `out.lang = Some(value.value());
            } else if meta.path.is_ident("lang_from") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lang_from = Some(value.value());
            } else if meta.path.is_ident("coord") {`
    );
    // still need FieldAttrs field - find lang: Option and add after
    derive = derive.replace(
      /(\/\/\/ `\#\[dsl\(lang = "jack"\)\`].*?\n\s*lang: Option<String>,)/s,
      `$1\n    /// \`#[dsl(lang_from = "language_id")]\` — fence language from a sibling Text field.\n    lang_from: Option<String>,`
    );
  } else {
    derive = derive.replace(
      `out.lang = Some(value.value());
            } else if meta.path.is_ident("coord") {`,
      `out.lang = Some(value.value());
            } else if meta.path.is_ident("lang_from") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lang_from = Some(value.value());
            } else if meta.path.is_ident("coord") {`
    );
  }
  // FieldPlan
  if (!derive.includes("lang_from: Option<String>")) {
    console.error("FAILED to add FieldAttrs.lang_from");
  }
  // FieldPlan struct fields - look for lang: Option in FieldPlan
  const fp = derive.indexOf("struct FieldPlan");
  const fpSlice = derive.slice(fp, fp + 1200);
  console.log("FieldPlan slice has lang_from?", fpSlice.includes("lang_from"));
  if (fp >= 0 && !fpSlice.includes("lang_from")) {
    derive = derive.slice(0, fp) + fpSlice.replace(
      "lang: Option<String>,",
      "lang: Option<String>,\n    lang_from: Option<String>,"
    ) + derive.slice(fp + 1200);
  }
  derive = derive.replace(
    `lang: attrs.lang.clone(),
            coord: attrs.coord,`,
    `lang: attrs.lang.clone(),
            lang_from: attrs.lang_from.clone(),
            coord: attrs.coord,`
  );
  derive = derive.replace(
    `let FieldPlan { ident, id, key, positional, optional, kind, elem_ty, block, unit, angle, refs, defines, lang, coord, dir } = plan;`,
    `let FieldPlan { ident, id, key, positional, optional, kind, elem_ty, block, unit, angle, refs, defines, lang, lang_from, coord, dir } = plan;`
  );
  derive = derive.replace(
    `} else if let Some(l) = lang {
            Some(quote! { ::dsl::Shape::Embed(#l) })
        } else if *coord {`,
    `} else if let Some(from) = lang_from {
            Some(quote! { ::dsl::Shape::EmbedFrom(#from) })
        } else if let Some(l) = lang {
            Some(quote! { ::dsl::Shape::Embed(#l) })
        } else if *coord {`
  );
  writeFileSync(derivePath, derive);
  console.log("[DEBUG] derive patched");
} else console.log("[DEBUG] derive already patched");

let schemaSrc = readFileSync(schema, "utf8");
if (!schemaSrc.includes("EmbedFrom")) {
  schemaSrc = schemaSrc.replace(
    `    Embed(&'static str),`,
    `    Embed(&'static str),
    /// Fence language taken from a sibling Text field named by this key (see \`#[dsl(lang_from)]\`).
    EmbedFrom(&'static str),`
  );
  writeFileSync(schema, schemaSrc);
  console.log("[DEBUG] schema EmbedFrom added — still need parse/print arms");
} else console.log("[DEBUG] schema already has EmbedFrom");
