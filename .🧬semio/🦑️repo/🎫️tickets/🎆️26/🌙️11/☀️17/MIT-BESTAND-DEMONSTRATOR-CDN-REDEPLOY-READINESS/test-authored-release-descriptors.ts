import { expect, test } from "bun:test";
import { authoredReleaseDigestV1 } from "./authored-release-descriptors.ts";
import cases from "./🔣️authored-release-cases.json";
import { APP_CHANNEL_VERSION } from "../../../../../../../🧰️framework/🔨️modules/🧾️app-channel/🔢️request-sequence/🟦️.ts";
import { encodePackValue } from "../../../../../../../🧰️framework/🔨️modules/🎁️packs/🟦️.ts";

for (const row of cases) {
  test(row.name, async () => {
    const target = {pluginId: row.pluginId, packageId: `semio:${row.pluginId}`, role: "plugin"} as const;
    const value = (version: string) => ({
      descriptorVersion: 1,
      role: target.role,
      packageId: target.packageId,
      manifest: {pluginId: target.pluginId, label: target.pluginId, version, apps: [], examples: []},
      execution: "isolated",
      executionProtocol: {appChannelVersion: APP_CHANNEL_VERSION},
      quotas: {},
      contributions: {},
    });
    const confirmed = encodePackValue(value(row.confirmedVersion));
    const authored = value(row.authoredVersion);
    const canonical = encodePackValue(authored);
    const json = row.formatted ? "  " + JSON.stringify(Object.fromEntries(Object.entries(authored).reverse()), null, 4) + "\r\n" : JSON.stringify(authored);
    const recordedDigest = Buffer.from(await crypto.subtle.digest("SHA-256", confirmed)).toString("hex");
    const authoredDigest = authoredReleaseDigestV1(target, Buffer.from(json), canonical);
    expect(authoredDigest).toBe(Buffer.from(await crypto.subtle.digest("SHA-256", canonical)).toString("hex"));
    expect(authoredDigest === recordedDigest).toBe(row.accepted);
    expect(() => authoredReleaseDigestV1({...target, pluginId: "foreign"}, Buffer.from(json), canonical)).toThrow("does not match its release owner");
    if (!row.accepted) expect(() => authoredReleaseDigestV1(target, Buffer.from(json), confirmed)).toThrow("forms disagree");
  });
}
