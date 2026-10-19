import { workspacePackages } from "../lay6-tools/pkgs.ts";
for (const p of workspacePackages()) if (process.argv.slice(2).some(n => p.name === n)) console.log(p.name, p.owner, "| sourceOwner", p.sourceOwner, "\n  ", Object.entries(p.exports).map(([k, v]) => `${k} -> ${v}`).join("\n   "));
