//#region 🔖kindNameFromRepresentation
/** @emoji 🏷️ Derives human-readable kind names from representation file stems (shape · role · family). */

/** @emoji 📎 Drops mesh extension, collider, and LOD scale suffixes from a file name. */
export function fileStemForNaming(fileName: string): string {
  return fileName
    .replace(/\.(glb|3dm|svg|png|jpg|webp)$/i, "")
    .replace(/_collider$/i, "")
    .replace(/_1to\d+$/i, "");
}

function titleizeToken(token: string): string {
  if (token === "\\") return "Backslash";
  if (token === "/") return "Slash";
  if (token.length === 1) return token.toUpperCase();
  return token.charAt(0).toUpperCase() + token.slice(1).toLowerCase();
}

const STOREFY_LEADERS = new Set(["first", "last", "single"]);

/** @emoji 🏷️ Titleized kind name with storey phrases grouped and Tambour moved after the storey role. */
export function typeNameFromFileName(fileName: string): string {
  const stem = fileStemForNaming(fileName);
  if (!stem) return "";
  const tokens = stem.split(/[-_]+/).filter((token) => token.length > 0);
  let storeyIdx = -1;
  for (let index = 0; index < tokens.length - 1; index += 1) {
    const lead = tokens[index]!.toLowerCase();
    const tail = tokens[index + 1]!.toLowerCase();
    if (STOREFY_LEADERS.has(lead) && tail === "storey") {
      storeyIdx = index;
      break;
    }
  }
  if (storeyIdx >= 0) {
    const beforeStorey = tokens.slice(0, storeyIdx);
    const prefix = beforeStorey.filter((token) => token.toLowerCase() !== "tambour").map(titleizeToken);
    const includesTambour =
      beforeStorey.some((token) => token.toLowerCase() === "tambour") ||
      tokens.slice(storeyIdx + 2).some((token) => token.toLowerCase() === "tambour");
    const storeyPhrase = `${titleizeToken(tokens[storeyIdx]!)} Storey`;
    const afterStorey = tokens
      .slice(storeyIdx + 2)
      .filter((token) => token.toLowerCase() !== "tambour")
      .map(titleizeToken);
    const parts = [...prefix, storeyPhrase];
    if (includesTambour) parts.push("Tambour");
    parts.push(...afterStorey);
    return parts.join(" ");
  }
  return tokens.map(titleizeToken).join(" ");
}
//#endregion 🔖kindNameFromRepresentation
