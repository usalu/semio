import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";

const fw = readdirSync(".").find(n => n.includes("framework") && !n.includes("\uFFFD"));

function find(base, pred, out = []) {
  for (const ent of readdirSync(base, { withFileTypes: true })) {
    const p = join(base, ent.name);
    if (ent.isDirectory() && !["node_modules", "target", ".git"].includes(ent.name)) find(p, pred, out);
    else if (ent.isFile() && pred(p)) out.push(p);
  }
  return out;
}

const changes = [];
function patch(path, transform) {
  const before = readFileSync(path, "utf8");
  const after = transform(before);
  if (after !== before) {
    writeFileSync(path, after);
    changes.push(path);
    console.log("PATCHED", path);
  } else {
    console.log("NOCHANGE", path);
  }
}

// 1) Label breadcrumb comment
{
  const label = find(join(fw, "🔨️modules/🖱️ui"), (p) => p.includes("/🏷️Label/") && p.endsWith("component.tsx"))[0];
  patch(label, (t) =>
    t.replace(/\/\/ \[🏘️compose[^\n]*/, "// [🖱️ui⚛️react/Label](repo://framework/modules/ui/elements/core/Label)"),
  );
}

// 2) UI README
{
  const readme = find(join(fw, "🔨️modules/🖱️ui"), (p) => p.endsWith("README.md") && p.includes("⚛️react"))[0];
  patch(readme, (t) =>
    t
      .replace(/Reusable compose UI elements/, "Reusable UI elements")
      .replace(
        /- The `ui` bundle owns the shared element source formerly embedded in `compose\/js\/sketchpad`\./,
        "- The `ui` bundle owns the shared element source for framework-facing packages.",
      ),
  );
}

// 3) PresentationDeck
{
  patch(".storybook/stories/animate/PresentationDeck.stories.tsx", (t) =>
    t.replace("cad · coda · animate · compose/algorithm", "cad · coda · animate · puzzle/algorithm"),
  );
}

// 4) .env.example blob root
{
  const env = find(join(fw, "🛍️products/🦑️repo"), (p) => p.endsWith(".env.example") && p.includes("coordinator"))[0];
  if (env) patch(env, (t) => t.replace("UPLOAD_BLOB_ROOT=/srv/compose/blob", "UPLOAD_BLOB_ROOT=/srv/semio/blob"));
}

// 5) Remove dead compose-js import comment in vscode extension
{
  const ext = find(
    join(fw, "🛍️products/🦑️repo"),
    (p) => p.endsWith("extension.ts") && p.includes("vscode") && !p.includes(".test."),
  )[0];
  if (ext)
    patch(ext, (t) =>
      t.replace(/\/\/ import \{ deserializeKit, Problem, validateKit \} from "@semio-tech\/compose-js\/compose";\n?/, ""),
    );
}

// 6) Asset fixture path comments
{
  const fixtures = find(join(fw, "🛍️products/🦑️repo/🖼️assets"), (p) => {
    try {
      return /compose\/asset/.test(readFileSync(p, "utf8"));
    } catch {
      return false;
    }
  });
  for (const p of fixtures) {
    patch(p, (t) => t.replaceAll("compose/asset/repo/", "framework/products/repo/assets/fixtures/"));
  }
}

// 7) bootstrap neo4j dest dir
{
  const sh = find(join(fw, "🛍️products/🦑️repo"), (p) => p.endsWith("script.sh") && p.includes("bootstrap"))[0];
  if (sh)
    patch(sh, (t) =>
      t.replace("${HOME}/.local/share/compose/neo4j-desktop", "${HOME}/.local/share/semio/neo4j-desktop"),
    );
}

console.log("\nCHANGED_COUNT", changes.length);
for (const c of changes) console.log(" -", c);
