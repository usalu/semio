import { expect, mock, test } from "bun:test";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";

const root = process.cwd();
const xlsx = resolve(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx");
const provider = resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts");

test("the normal XLSX Rust target runs the public, editor and Strict mutation suites", async () => {
  const runArtifactRustPackageMain = mock(async (_directory: string, _name: string, _options: { testPackages?: string[] }) => {});
  mock.module(provider, () => ({ runArtifactRustPackageMain }));
  await import(resolve(xlsx, "📦️packages/🦀️rust/📜️script.ts"));

  const members = [
    "📦️packages/🦀️rust",
    "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📦️packages/🦀️rust",
    "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/📦️packages/🦀️rust",
    "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/📦️packages/🦀️rust",
    "🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/📦️packages/🦀️rust",
  ];
  const { workspace } = Bun.TOML.parse(await readFile(resolve(xlsx, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } };
  const names = await Promise.all(members.map(async member => {
    expect(workspace.members).toContain(member);
    const manifest = Bun.TOML.parse(await readFile(resolve(xlsx, member, "Cargo.toml"), "utf8")) as { package: { name: string } };
    return manifest.package.name;
  }));
  expect(runArtifactRustPackageMain).toHaveBeenCalledTimes(1);
  const [packageRoot, name, options] = runArtifactRustPackageMain.mock.calls[0]!;
  expect(packageRoot).toBe(resolve(xlsx, members[0]!));
  expect(name).toBe(names[0]);
  expect(options.testPackages).toEqual(names);
});
