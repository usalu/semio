
  const fs = require("fs");
  const { join } = require("path");

  const mathDir = fs.readdirSync("🧰️framework/🔨️modules").find((m) => m.includes("math"));
  if (!mathDir) throw new Error("math dir not found");
  const mathPkgRel = "🧰️framework/🔨️modules/" + mathDir + "/📦️packages/🦀️rust";
  if (!fs.existsSync(join(mathPkgRel, "Cargo.toml"))) throw new Error("math package missing: " + mathPkgRel);

  let cargo = fs.readFileSync("Cargo.toml", "utf8");
  const before = cargo;

  function escapeRegExp(s) {
    return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  }

  const mathImplMemberRe = new RegExp(
    "^\\s*\"" + escapeRegExp("🧰️framework/🔨️modules/" + mathDir + "/") + "[^\"]*⚡️implementations/🦀️rust\",?\\s*\\n",
    "gm",
  );
  const removedMathMembers = [...cargo.matchAll(mathImplMemberRe)].length;
  cargo = cargo.replace(mathImplMemberRe, "");

  if (!cargo.includes(mathPkgRel)) {
    const anchor = '    "🌎️hub/📦️packages/🦀️rust",';
    if (cargo.includes(anchor)) {
      cargo = cargo.replace(anchor, '    "' + mathPkgRel + '",\n' + anchor);
    } else {
      cargo = cargo.replace(/members\s*=\s*\[\n/, (m) => m + '    "' + mathPkgRel + '",\n');
    }
  }

  const mathAliasRe = /^semio-framework-os-kernel-math-[^\n]+\n/gm;
  const removedMathAliases = [...cargo.matchAll(mathAliasRe)].length;
  cargo = cargo.replace(mathAliasRe, "");

  if (!/^semio-framework-math\s*=/m.test(cargo)) {
    cargo = cargo.replace(
      /\[workspace\.dependencies\]\n/,
      '[workspace.dependencies]\nsemio-framework-math = { path = "' + mathPkgRel + '" }\n',
    );
  }

  if (cargo.includes("undefined")) throw new Error("refusing to write Cargo.toml containing undefined");
  if (cargo.includes(",,")) throw new Error("refusing to write Cargo.toml containing double commas");

  fs.writeFileSync("Cargo.toml", cargo);
  console.log(
    JSON.stringify(
      {
        mathDir,
        removedMathMembers,
        removedMathAliases,
        lenBefore: before.length,
        lenAfter: cargo.length,
        hasMathPkgMember: cargo.includes(mathPkgRel),
      },
      null,
      2,
    ),
  );
