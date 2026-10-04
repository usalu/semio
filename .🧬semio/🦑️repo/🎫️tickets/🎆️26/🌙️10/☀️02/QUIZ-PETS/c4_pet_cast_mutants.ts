/** 🧟️ Ticket tool of work package C4: the site suite `🐾️pet-cast` run against one broken copy of the architecture menagerie, to show that its checks of states, tricks, gear and chemistry are not vacuous. `C4_MUTANT` names the break; every read of a pets document goes through it, the documents on disk stay untouched.
 *
 * Usage (from `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript`):
 *   C4_MUTANT=<name> bun ../../../../../node_modules/vitest/vitest.mjs run --config "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c4_pet_cast_mutants.config.ts"
 */
import { vi } from "vitest";

vi.mock("node:fs", async (importOriginal) => {
  const fs = await importOriginal<typeof import("node:fs")>();
  type Json = Record<string, unknown> & { [key: string]: any };
  const breaks: Record<string, { readonly file: string; readonly change: (document: Json) => void }> = {
    "trick-clip": { file: "☀️sunny", change: (sunny) => void (sunny.tricks[0].clip = "nowhere") },
    "orphan-state": { file: "☀️sunny", change: (sunny) => void sunny.states.push({ id: "eclipse", name: { en: "Eclipse", de: "Finsternis" }, lasts: 10, then: "shining" }) },
    "dead-end": { file: "🖥️servy", change: (servy) => void (servy.states.push({ id: "crashed", name: { en: "Crashed", de: "Abgestürzt" } }), (servy.tricks.find((trick: Json) => trick.id === "reboot").to = "crashed")) },
    "no-circle": { file: "🌀️pumpy", change: (pumpy) => void (pumpy.tricks.find((trick: Json) => trick.id === "whisper").from = ["heating"]) },
    "no-glide": { file: "💨️windy", change: (windy) => void delete windy.repertoire.glide },
    "rule-missing": { file: "", change: (ensemble) => void (ensemble.chemistry = ensemble.chemistry.filter((reaction: Json) => !String(reaction.id).startsWith("r33"))) },
    "held-too-long": { file: "", change: (ensemble) => void (ensemble.chemistry.find((reaction: Json) => reaction.id === "r05").when.held = 30) },
    "trick-not-offered": { file: "", change: (ensemble) => void (ensemble.chemistry.find((reaction: Json) => reaction.id === "r07-shining").near.state = "dim") },
    "too-close": { file: "", change: (ensemble) => void (ensemble.chemistry.find((reaction: Json) => reaction.id === "r01-heavy").within = 10) },
    "idle-effect": { file: "", change: (ensemble) => void ensemble.chemistry.find((reaction: Json) => reaction.id === "r02").then.push({ on: "near" }) },
  };
  const chosen = breaks[process.env.C4_MUTANT ?? ""];
  if (chosen === undefined) throw new Error(`C4_MUTANT must be one of ${Object.keys(breaks).join(", ")}`);
  const target = chosen.file === "" ? /🐾️pets[\\/]🔣️\.json$/u : new RegExp(`${chosen.file}[\\\\/]🔣️\\.json$`, "u");
  const readFileSync = ((path: Parameters<typeof fs.readFileSync>[0], options?: Parameters<typeof fs.readFileSync>[1]) => {
    const text = fs.readFileSync(path, options as never) as unknown as string;
    if (!target.test(String(path))) return text;
    const document = JSON.parse(text) as Json;
    chosen.change(document);
    return JSON.stringify(document);
  }) as typeof fs.readFileSync;
  return { ...fs, readFileSync, default: { ...fs, readFileSync } };
});

await import("../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🐾️pet-cast/🟦️.ts");
