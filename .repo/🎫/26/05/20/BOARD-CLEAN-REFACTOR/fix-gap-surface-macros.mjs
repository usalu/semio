import { readFileSync, writeFileSync } from "node:fs";

const path = "c:/git/compose/compose/client/lib/rs/lib.rs";
const lines = readFileSync(path, "utf8").split(/\r?\n/);

function extractFirstNamesBlock(startLine, macroName) {
  let i = startLine;
  while (i < lines.length && !lines[i].includes(`macro_rules! ${macroName}`)) i++;
  if (i >= lines.length) throw new Error(`macro ${macroName} not found`);
  while (i < lines.length && !lines[i].includes("(@names)")) i++;
  i++;
  const names = [];
  while (i < lines.length) {
    const t = lines[i].trim();
    if (t === "};") break;
    const m = t.match(/^([A-Za-z][A-Za-z0-9]*),?$/);
    if (m) names.push(m[1]);
    i++;
  }
  return names;
}

const familyStart = lines.findIndex((l) => l.includes("macro_rules! gap_surface_family_name_list"));
const relayStart = lines.findIndex((l) => l.includes("macro_rules! gap_surface_existing_relay_name_list"));
const tailStart = lines.findIndex((l, i) => i > relayStart && l.includes("macro_rules! with_gap_surface_family_names"));

const familyNames = extractFirstNamesBlock(familyStart, "gap_surface_family_name_list");
const relayNames = extractFirstNamesBlock(relayStart, "gap_surface_existing_relay_name_list");

const identLines = (names) => names.map((n) => `        ${n},`).join("\n");

const block = `    macro_rules! __gap_surface_family_name_idents {
        () => {
${identLines(familyNames)}
        };
    }

    macro_rules! __gap_surface_existing_relay_name_idents {
        () => {
${identLines(relayNames)}
        };
    }

    macro_rules! gap_surface_families {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_family!($Name);)+
        };
    }

    macro_rules! gap_surface_existing_relays {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_existing_relay!($Name);)+
        };
    }

    #[macro_export]
    macro_rules! gap_surface_family_name_list {
        (@names) => { $crate::schema_gap_surfaces::__gap_surface_family_name_idents!() };
        (@apply_families) => {
            $crate::schema_gap_surfaces::gap_surface_families! {
                $crate::schema_gap_surfaces::__gap_surface_family_name_idents!()
            }
        };
        (@register $builder:expr) => {
            $crate::register_gap_surface_family_connections! {
                @expand $builder;
                $crate::schema_gap_surfaces::__gap_surface_family_name_idents!()
            }
        };
    }

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {
        (@names) => { $crate::schema_gap_surfaces::__gap_surface_existing_relay_name_idents!() };
        (@apply_relays) => {
            $crate::schema_gap_surfaces::gap_surface_existing_relays! {
                $crate::schema_gap_surfaces::__gap_surface_existing_relay_name_idents!()
            }
        };
        (@register $builder:expr) => {
            $crate::register_gap_surface_existing_relay_connections! {
                @expand $builder;
                $crate::schema_gap_surfaces::__gap_surface_existing_relay_name_idents!()
            }
        };
    }

`;

const insertAt = lines.findIndex((l) => l.includes("macro_rules! gap_surface_families"));
const out = [...lines.slice(0, insertAt), ...block.split("\n"), ...lines.slice(tailStart)];

writeFileSync(path, out.join("\n"), "utf8");
console.log("ok", { familyNames: familyNames.length, relayNames: relayNames.length, insertAt, tailStart });
