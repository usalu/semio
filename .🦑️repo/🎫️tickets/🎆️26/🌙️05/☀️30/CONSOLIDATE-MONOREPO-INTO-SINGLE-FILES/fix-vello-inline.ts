import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const libPath = join(root, "infinite/canvas/vello/lib.rs");
const ticket = import.meta.dirname;
const iconCodec = readFileSync(join(ticket, "icon_codec.rs.recovered"), "utf8").trimEnd();
const theme = readFileSync(join(ticket, "theme.rs.recovered"), "utf8").trimEnd();

let lib = readFileSync(libPath, "utf8");
const old = `// #region 🔖️IconCodec
pub mod icon_codec;
pub use icon_codec::{board_resolve_icon_kind, board_typst_markup_to_svg, decode_icon, encode_icon, BoardResolvedIcon, Icon, ThemedSvgLookup};
pub mod theme;
// #endregion 🔖️IconCodec`;

const neu =
  "// #region 🔖️IconCodec\n" +
  "pub mod icon_codec {\n// #region icon_codec\n" +
  iconCodec +
  "\n// #endregion icon_codec\n}\n" +
  "pub use icon_codec::{board_resolve_icon_kind, board_typst_markup_to_svg, decode_icon, encode_icon, BoardResolvedIcon, Icon, ThemedSvgLookup};\n" +
  "pub mod theme {\n// #region theme\n" +
  theme +
  "\n// #endregion theme\n}\n" +
  "// #endregion 🔖️IconCodec";

if (!lib.includes(old)) throw new Error("vello IconCodec block not found");
writeFileSync(libPath, lib.replace(old, neu));
console.log("fixed vello inline modules");
