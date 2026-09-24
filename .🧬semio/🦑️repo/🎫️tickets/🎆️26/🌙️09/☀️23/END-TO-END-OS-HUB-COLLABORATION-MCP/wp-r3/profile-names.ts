import { mutationCatalogProblems } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const names = [["🔖️v1","✉️base"],["9️⃣89a","🧱️base"],["4️⃣1.4","♾️any"],["🔖️ap214","1️⃣cc1"],["🔖️1","🌐️any"],["🔖️1","✳️any"],["🔖️1","any"],["v1","✉️base"]];
for (const [s, u] of names) {
  const owner = `x/🏅️standards/${s}/🪆️subsets/${u}`;
  console.log(s, u, mutationCatalogProblems({ id: "a", capability: "b", standardDirectoryName: s, subsetDirectoryName: u, kinds: ["k"], vectors: [] }, owner).join("; "));
}
