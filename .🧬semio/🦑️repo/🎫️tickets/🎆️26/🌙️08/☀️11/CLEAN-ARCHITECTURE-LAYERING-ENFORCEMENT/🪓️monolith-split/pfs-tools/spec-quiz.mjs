import fs from "node:fs";
const T = process.env.T;
const file = "🧰️framework/🛍️products/❓️quiz/🧪️tests/🚶️learner-journey/🟦️.tsx";
const orig = fs.readFileSync(process.env.G + "/pfs-orig/learner-journey.tsx.txt", "utf8");
const header = orig.slice(orig.indexOf("/**") + 3, orig.indexOf("*/")).replace(/^ \*\n?/gm, "").trim();
const body = orig.slice(orig.indexOf("/**"), orig.indexOf("*/") + 2).split("\n").slice(1, -1).join("\n");
const kitDoc = `/** 🧰️ Shared kit of the learner-journey suites (the topics sit in the sibling folders \`🚶️journey-and-tabs\`, \`🫡️deputy-and-contract\`, \`⛰️challenge-session\` and \`🧷️run-truth\`): the quiz and the catalog, the proctor double and the builders of sessions, runs and answers. Together the suites check the whole learner journey against a proctor double speaking the framework wire (design §9a) and deciding with\n${body}\n *\n * @see ../../🎯️targets/⚛️react/🟦️.tsx — the client under test\n */`;
const doc = (emoji, text, extra = "") => `/** ${emoji} ${text}\n *\n * @see ../🧰️learner-journey-kit/🟦️.tsx — the shared quiz, proctor double and builders\n * @see ../../🎯️targets/⚛️react/🟦️.tsx — the client under test${extra}\n */`;
const spec = {
  file,
  label: "learner-journey",
  childRoot: "🧰️framework/🛍️products/❓️quiz/🧪️tests",
  deleteParent: true,
  children: [
    { folder: "🧰️learner-journey-kit", doc: kitDoc.replace("🧰️ Shared kit of the learner-journey suites", "🧰️ Shared kit of the learner-journey suites"), stmts: ["96-456", "710-733", "1088-1121", "1528-1561"] },
    { folder: "🚶️journey-and-tabs", doc: doc("🚶️", "The whole learner journey from the introduction through a submitted run to the leaderboard and a reload that resumes in the chosen language, several tabs, cancellation and other devices, and presence through the whole app."), stmts: ["458", "735", "942"] },
    { folder: "🫡️deputy-and-contract", doc: doc("🫡️", "The deputy while the proctor is away, a proctor of another contract and a screen that fails."), stmts: ["1123", "1352", "1359", "1498"] },
    { folder: "⛰️challenge-session", doc: doc("⛰️", "The challenges through the session: the sheet of each, switching voids the open run, the clock of an expert run, the hints of an easy run, points by challenge and stored runs restored only in the shape of the challenges."), stmts: ["1563"] },
    { folder: "🧷️run-truth", doc: doc("🧷️", "What the device shows of a run stays true to its answers: hints never outlive the answer they were given for, another tab's copy merges, a refused answer is read again and an answer is judged at the instant of its own last edit."), stmts: ["1884-1912"] },
  ],
};
fs.writeFileSync(T + "/spec-quiz.json", JSON.stringify(spec, null, 1));
