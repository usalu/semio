const R = "/Users/ueli/Documents/semio";
const H = `${R}/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel`;
const f = "\u{1F7E6}️.ts";
const FE = "️";
const child = (emoji: string, slug: string) => `${H}/${emoji}${FE}${slug}/${f}`;
const args = await import(child("\u{1F9ED}", "arguments"));
const state = await import(child("\u{1F9E0}", "state"));
const puzzle = await import(`${R}/✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/${f}`);
const out = `${process.env.TMPDIR ?? "/tmp"}/met-dry-parse`;
const signal = new AbortController().signal;
const snapshot = () => ({ renderer: state.renderer, serveOrigin: state.serveOrigin, only: state.only ? [...state.only] : null, reloadSelected: state.reloadSelected, locales: state.locales, chords: [...state.chordLocales], universalRoute: state.universalRoute, folderAt: state.folderAt, longHistory: state.longHistory, variant: state.scenario?.variant ?? null, route: args.routeUrl(), ndjson: state.ndjsonPath.endsWith(".ndjson") });
args.configure(["--renderer", "react", "--serve", "http://127.0.0.1:6012/", "--only", "1,2,reload", "--locales", "en", "--chords", "de", "--folder-at", "5", "--long-history", "50", "--out", out], out, signal, puzzle.puzzle2dTimeTravelScenario);
console.log("react scenario:", JSON.stringify(snapshot()));
args.configure(["--renderer", "wgpu", "--explore", "--out", out], out, signal, puzzle.puzzle2dTimeTravelScenario);
console.log("wgpu default serve:", JSON.stringify(snapshot()));
args.configure(["--universal", "--variant", "draw", "--serve", "http://127.0.0.1:6064/", "--renderer", "react", "--out", out], out, signal, undefined);
console.log("universal without scenario:", JSON.stringify(snapshot()));
try { args.configure(["--renderer", "react", "--out", out], out, signal, undefined); console.log("NO THROW"); } catch (error) { console.log("no scenario and no --serve ->", String(error).slice(0, 120)); }
