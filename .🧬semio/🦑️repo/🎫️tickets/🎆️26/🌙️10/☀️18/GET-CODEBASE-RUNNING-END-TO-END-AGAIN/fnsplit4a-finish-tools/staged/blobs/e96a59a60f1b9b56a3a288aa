/** 🟠️ The part3 phase of transformTestClosure. */
import { identifierSet, prunedUse, useLeafNames } from "../../🌳️module-tree/🟦️.ts";
import { relativePosix } from "../../📝️render/🟦️.ts";
import type { useResultState } from "../🔶️result/🟦️.ts";
import { absolutizeSuperPaths, declaredNames, discoverMounts, hostPrivate, prependAfterInner } from "../🟢️shared/🟦️.ts";
import type { TestTransformOptions } from "../🟢️shared/🟦️.ts";

/** 🟡 Runs the effects and registrations of transformTestClosure. */
export function usePart3({ options, rootFile, result, readText, subtreeText, widenFor, hostGlobCarry, setText, retargetPrivateImportPaths, carryIntoHostGlobModules, repoRoot, flushEdits }: { readonly options: TestTransformOptions; readonly rootFile: string; readonly repoRoot: string } & Pick<ReturnType<typeof useResultState>, "result" | "readText" | "subtreeText" | "widenFor" | "hostGlobCarry" | "setText" | "retargetPrivateImportPaths" | "carryIntoHostGlobModules" | "flushEdits">) {
  if (options.detach) {
    const { mounts } = discoverMounts(rootFile, result.rootSource, readText);
    for (const mount of mounts) {
      if (mount.hostIsRoot) continue;
      const body = readText(mount.testFile);
      if (body === null) continue;
      const whole = subtreeText(mount.testFile);
      const rawPrivateUse = hostPrivate(readText(mount.hostFile) ?? "", identifierSet(whole), declaredNames(body, true), whole, mount.ownerPath);
      const stuck = rawPrivateUse.relative.filter((statement) => !/^use\s+super::/u.test(statement) || /\*\s*;\s*$/u.test(statement));
      const relativeCarry = rawPrivateUse.relative.filter((statement) => !stuck.includes(statement)).map((statement) => absolutizeSuperPaths(statement, mount.ownerPath));
      if (relativeCarry.some((statement) => statement === null)) continue;
      const privateUse = { ...rawPrivateUse, relative: stuck, carry: [...rawPrivateUse.carry, ...(relativeCarry as string[])] };
      const hasSuper = /\bsuper::/u.test(body);
      if (privateUse.relative.length || (!hasSuper && !privateUse.items.length)) continue;
      const absolute = hasSuper ? absolutizeSuperPaths(body, [...mount.ownerPath, mount.node.name]) : body;
      if (absolute === null || (absolute === body && !privateUse.items.length)) continue;
      const widenedUse = widenFor(mount, privateUse.items, body);
      if (widenedUse === null) continue;
      const rawGlobCarry = hostGlobCarry(mount, absolute);
      if (rawGlobCarry === null) continue;
      const known = new Set([...privateUse.carry.flatMap((statement) => useLeafNames(statement)), ...declaredNames(body)]);
      const globCarry = rawGlobCarry.flatMap((statement) => {
        const leaves = useLeafNames(statement);
        if (!leaves.length) return [`#[allow(unused_imports)]\n${statement}`];
        const fresh = leaves.filter((leaf) => !known.has(leaf));
        return fresh.length ? [`#[allow(unused_imports)]\n${prunedUse(statement, fresh)}`] : [];
      });
      setText(mount.testFile, prependAfterInner(retargetPrivateImportPaths(carryIntoHostGlobModules(absolute, mount, privateUse.carry), mount, privateUse.carry), [...privateUse.carry, ...globCarry, ...widenedUse]));
      result.notes.push(`made ${relativePosix(repoRoot, mount.testFile)} independent of its host module (super paths absolute${privateUse.carry.length ? ", private absolute imports carried" : ""}${privateUse.items.length ? `, widened ${privateUse.items.join(", ")} to pub` : ""})`);
    }
    flushEdits();
  }
}
