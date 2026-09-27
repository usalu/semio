/** 🧟️ C13: the access fixture discriminates — the pre-fix behaviour (every shared opening requests the preference default, the
 * editor) and a role-blind "always viewer" mutant each fail the cases they must fail. */
import fixture from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🧫️fixtures/📍️scope/🔣️.json";
import { sharedDocumentOpeningRoleV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🟦️.ts";
const mutants: Record<string, (row: (typeof fixture.accessCases)[number]) => string | null> = {
  "pre-fix (preference default = editor)": () => "editor",
  "always viewer": () => "viewer",
  "fix": (row) => sharedDocumentOpeningRoleV1(row.events as never, row.spaceId, row.userId),
};
for (const [name, decide] of Object.entries(mutants)) {
  const failing = fixture.accessCases.filter((row) => decide(row) !== row.role).map((row) => row.id);
  console.log(`${name}: ${fixture.accessCases.length - failing.length}/${fixture.accessCases.length} pass; failing ${JSON.stringify(failing)}`);
}
