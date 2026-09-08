#!/usr/bin/env bun
/** 🩺️ Clause-by-clause diagnosis of `@semio-tech/puzzle-js`'s `publication-authority-audit`
 * `ownerOracle`: the audit only reports "diverged from the fixture", so this reads the very same
 * anchors out of `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` and names each
 * clause that is false for each owner. Diagnostic only — never a gate. */
import { resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const puzzleRoot = resolve(repoRoot, "✏️s/🔌️plugins/🧩️puzzle");
const auditFile = resolve(puzzleRoot, "📦️packages/🟦️typescript/📜️script.ts");
const audit = await Bun.file(auditFile).text();
const fixture = await Bun.file(resolve(puzzleRoot, "🔏️publication-authority/🔣️.json")).json() as {
  owners: { owner: string; source: string; groups: { status: string; lanes: string[]; routes: string[] }[] }[];
};

const reserved5d = new Set(["copy", "cut", "paste", "import-media"]);
const reserved2d = new Set(["import-media"]);
const quotedValues = (source: string): string[] => [...source.matchAll(/"([^"]+)"/g)].map((match) => match[1]!);
const exactArray = (left: string[], right: string[]): boolean =>
  JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;

/** 🔎️ Every `production.includes("…")` / `!production.includes("…")` anchor of one owner branch. */
function branchAnchors(owner: string): { anchor: string; negated: boolean }[] {
  const marker = `if (owner.owner === ${JSON.stringify(owner)}) {`;
  const shared = audit.slice(audit.indexOf("const exactFactory ="), audit.indexOf("if (!exactFactory"));
  const start = audit.indexOf(marker);
  const branch = start < 0 ? "" : audit.slice(start, audit.indexOf("\n  }", start));
  const tail = owner === "Puzzle5dPlayApp" ? audit.slice(audit.indexOf('if (owner.owner !== "Puzzle5dPlayApp") return true;'), audit.indexOf("\nclass PublicationAuthorityAuditScript")) : "";
  const scope = `${shared}\n${branch}\n${tail}`;
  return [...scope.matchAll(/(!?)production\.includes\((?:"((?:[^"\\]|\\.)*)"|'((?:[^'\\]|\\.)*)')\)/g)].map((match) => ({
    anchor: match[2] === undefined ? match[3]!.replace(/\\'/g, "'") : JSON.parse(`"${match[2]}"`) as string,
    negated: match[1] === "!",
  }));
}

for (const owner of fixture.owners) {
  const source = await Bun.file(resolve(puzzleRoot, owner.source)).text();
  const production = source.split("//#region 🧪️Testkit")[0]!;
  const failures: string[] = [];
  const dimension = owner.owner.slice(6, 8).toUpperCase();
  const appGroups = owner.groups.map((group) => ({
    ...group,
    routes: group.routes.filter((route) => (owner.owner !== "Puzzle5dPlayApp" || !reserved5d.has(route)) && (owner.owner !== "Puzzle2dPlayApp" || !reserved2d.has(route))),
  }));
  const appRoutes = appGroups.flatMap((group) => group.routes);
  const migrated = appGroups.filter((group) => group.status === "migrated").flatMap((group) => group.routes);
  const pairs = new Map([...production.matchAll(/\.action_interactive_job\((?:"([^"]+)"|set_fill_count::STEP_ACTION_ID),\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((match) => [match[1] ?? "setFillCountStep", match[2]!]));
  const expectedPairs = new Map(appGroups.flatMap((group) => group.routes.map((route) => [route, group.status === "migrated" ? "Migrated" : "BatchOnlyPendingRewrite"])));
  if (!exactArray([...pairs.keys()], appRoutes)) {
    failures.push(`manifest bijection: onlySource=${[...pairs.keys()].filter((id) => !appRoutes.includes(id)).join(",")} onlyFixture=${appRoutes.filter((id) => !pairs.has(id)).join(",")}`);
  }
  const wrongStatus = appRoutes.filter((route) => pairs.has(route) && pairs.get(route) !== expectedPairs.get(route));
  if (wrongStatus.length) failures.push(`status disagreement: ${wrongStatus.map((route) => `${route}=${pairs.get(route)}!=${expectedPairs.get(route)}`).join(",")}`);
  const retainedMatch = production.match(new RegExp(`PUZZLE${dimension}_RETAINED_TOOL_IDS: &\\[&str\\] = &\\[([\\s\\S]*?)\\];`));
  const retained = retainedMatch ? quotedValues(retainedMatch[1]!) : [];
  if (!retainedMatch) failures.push("retained id declaration missing");
  else if (!exactArray(retained, migrated)) failures.push(`retained set: onlySource=${retained.filter((id) => !migrated.includes(id)).join(",")} onlyFixture=${migrated.filter((id) => !retained.includes(id)).join(",")}`);
  const factory = `${owner.owner.slice(0, 8)}RetainedCommandJobFactory`;
  const factoryBlock = production.split(new RegExp(`impl (?:[A-Za-z_][A-Za-z0-9_]*::)*ArtifactOwnedToolJobFactory for ${factory}\\b`))[1]?.split("//#endregion 🧵️RetainedCommands")[0] ?? "";
  if (!factoryBlock) failures.push(`factory block for ${factory} not found`);
  const contracts = new Map([...factoryBlock.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)].map((match) => [
    match[1]!,
    [...match[2]!.matchAll(/ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|Child|HostOnly)/g)].map((lane) => lane[1]!),
  ]));
  const laneVariant = (lane: string): string => lane.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");
  const expectedContracts = new Map(appGroups.filter((group) => group.status === "migrated").flatMap((group) => group.routes.map((route) => [route, group.lanes.map(laneVariant)] as const)));
  if (!exactArray([...contracts.keys()], [...expectedContracts.keys()])) {
    failures.push(`contract bijection: onlySource=${[...contracts.keys()].filter((id) => !expectedContracts.has(id)).join(",")} onlyFixture=${[...expectedContracts.keys()].filter((id) => !contracts.has(id)).join(",")}`);
  }
  const laneMismatch = [...expectedContracts].filter(([route, lanes]) => contracts.has(route) && !exactArray(contracts.get(route)!, lanes));
  if (laneMismatch.length) failures.push(`contract lanes: ${laneMismatch.map(([route, lanes]) => `${route} source=[${contracts.get(route)!.join("|")}] fixture=[${lanes.join("|")}]`).join(" ; ")}`);
  const proofBlock = production.split("semio_framework_plugin::bounded_first_step_tool_proofs!")[1]?.split("fn register_tool_job_factories")[0] ?? "";
  if (!proofBlock) failures.push("bounded_first_step_tool_proofs! block not found");
  const proofIds = quotedValues(proofBlock.match(/tools:\s*\[([^\]]*)\]/)?.[1] ?? "");
  if (!exactArray(proofIds, migrated)) failures.push(`proof set: onlySource=${proofIds.filter((id) => !migrated.includes(id)).join(",")} onlyFixture=${migrated.filter((id) => !proofIds.includes(id)).join(",")}`);
  if (!new RegExp(`type Owner = (?:[A-Za-z_][A-Za-z0-9_]*::)*EditorApp<${owner.owner}>;`).test(production)) failures.push("type Owner alias regex");
  for (const trait of ["ToolJobFactory", "ArtifactOwnedToolJobFactory"]) {
    if (!new RegExp(`impl (?:[A-Za-z_][A-Za-z0-9_]*::)*${trait} for ${factory}\\b`).test(production)) failures.push(`impl ${trait} for ${factory}`);
  }
  if (!proofBlock.includes(`factory: "${factory}"`)) failures.push(`proof block factory: "${factory}"`);
  if (!proofBlock.includes(`factory_type: ${factory}`)) failures.push(`proof block factory_type: ${factory}`);
  const registration = owner.owner === "Puzzle5dPlayApp" ? `registry.register(${factory}::new(&controller_id))` : `registry.register(${factory}::new(&controller))`;
  if (!production.includes(registration)) failures.push(`missing anchor: ${JSON.stringify(registration)}`);
  if (owner.owner === "Puzzle5dPlayApp") {
    const guard = production.indexOf('if !["copy", "cut", "paste", "import-media"].contains(&request.tool_id.as_str())');
    const decode = production.indexOf("puzzle5d_preflight_reserved_wire", guard);
    if (!(guard >= 0 && decode > guard)) failures.push(`reserved guard order guard=${guard} decode=${decode}`);
    for (const reservedFactory of ["Puzzle5dCopyJobFactory", "Puzzle5dCutJobFactory", "Puzzle5dPasteJobFactory", "Puzzle5dImportJobFactory"]) {
      if (!production.includes(`registry.register(${reservedFactory}::new(&controller_id))`)) failures.push(`missing reserved registration: ${reservedFactory}`);
    }
    for (const [reservedFactory, route] of [["Puzzle5dCopyJobFactory", "copy"], ["Puzzle5dCutJobFactory", "cut"], ["Puzzle5dPasteJobFactory", "paste"], ["Puzzle5dImportJobFactory", "import-media"]] as const) {
      const anchor = `puzzle5d_reserved_factory!(${reservedFactory}, "${route}", "puzzle.5d.reserved.${route}.v1")`;
      if (!production.includes(anchor)) failures.push(`missing anchor: ${JSON.stringify(anchor)}`);
    }
    for (const route of ["cut", "paste", "import-media"]) {
      const anchor = `tool_id: "${route}", lanes: &[ArtifactToolPublicationLane::Artifact]`;
      if (!production.includes(anchor)) failures.push(`missing anchor: ${JSON.stringify(anchor)}`);
    }
  }
  for (const { anchor, negated } of branchAnchors(owner.owner)) {
    const present = production.includes(anchor);
    if (present === negated) failures.push(`${negated ? "must NOT contain" : "missing anchor"}: ${JSON.stringify(anchor)}`);
  }
  console.log(`\n===== ${owner.owner} (${failures.length} failing clause${failures.length === 1 ? "" : "s"}) =====`);
  for (const failure of failures) console.log(`  ❌️ ${failure}`);
}
