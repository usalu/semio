// 🧪️Scratch check (used while designing policyNormalizeRelPath): simulate policyScopeKey/
// policyNormalizeRelPath against every real entry in POLICY_DIFF_COMPLETENESS_ALLOWLIST /
// POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST to verify the canonical <pluginId>/<component> (or
// <pluginId>/<appId>/<component> when disambiguation is needed) key scheme does not collide two
// distinct real files onto the same key. Confirmed 0 collisions after adding the file-suffix mechanism
// (see policyFileSuffix in the shipped file). Not part of the shipped file — ticket-scratch only.

function policyStripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}

const POLICY_COMPONENT_ALIASES: Record<string, string> = { protocol: "spr" };

function policyCanonicalComponent(segment: string): string {
  const ascii = policyStripEmoji(segment);
  return POLICY_COMPONENT_ALIASES[ascii] ?? ascii;
}

/** 🧵Non-default file inside a crate/component dir (e.g. `benches/protocol.rs`) stays distinguishable from its crate's default entry file (`📦️lib.rs` legacy, `🦀️component.rs` future) — dropped extension, emoji-stripped, dash-joined. */
function policyFileSuffix(tailSegments: string[], defaultFile: string): string {
  if (tailSegments.length === 1 && tailSegments[0] === defaultFile) return "";
  return `#${tailSegments.map((s) => policyStripEmoji(s.replace(/\.rs$/, ""))).join("-")}`;
}

function policyNormalizeRelPath(relPath: string): string {
  const norm = relPath.startsWith("./") ? relPath.slice(2) : relPath;
  const segments = norm.split("/");

  const implIdx = segments.indexOf("⚡️implementations");
  if (implIdx > 1) {
    const moduleSeg = segments[implIdx - 1]!;
    const suffix = policyFileSuffix(segments.slice(implIdx + 2), "📦️lib.rs");
    const ownerChain = segments.slice(0, implIdx - 1).filter((s) => s !== "🔨️modules" && s !== "🛂️manifest");
    const pluginsIdx = ownerChain.indexOf("🔌️plugins");
    if (pluginsIdx >= 0 && ownerChain.length > pluginsIdx + 1) {
      const pluginId = policyStripEmoji(ownerChain[pluginsIdx + 1]!);
      const appsIdx = ownerChain.indexOf("🎛️apps");
      const appId = appsIdx >= 0 && ownerChain.length > appsIdx + 1 ? policyStripEmoji(ownerChain[appsIdx + 1]!) : undefined;
      const component = policyCanonicalComponent(moduleSeg);
      return (appId && appId !== pluginId ? `${pluginId}/${appId}/${component}` : `${pluginId}/${component}`) + suffix;
    }
    const ownerId = policyStripEmoji(ownerChain[ownerChain.length - 1] ?? "");
    if (ownerId) return `${ownerId}/${policyCanonicalComponent(moduleSeg)}${suffix}`;
  }

  const artifactsIdx = segments.indexOf("🗿️artifacts");
  if (artifactsIdx > 0 && segments.length > artifactsIdx + 2) {
    const pluginId = policyStripEmoji(segments[artifactsIdx - 1] ?? "");
    const artifactId = policyStripEmoji(segments[artifactsIdx + 1] ?? "");
    const component = policyCanonicalComponent(segments[artifactsIdx + 2]!);
    const suffix = policyFileSuffix(segments.slice(artifactsIdx + 3), "🦀️component.rs");
    return (artifactId && artifactId !== pluginId ? `${pluginId}/${artifactId}/${component}` : `${pluginId}/${component}`) + suffix;
  }

  return norm;
}

const DIFF = [
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️document/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🧪️testkit/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🧪️testkit/⚡️implementations/🦀️rust/benches/protocol.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🔀️crdt/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🎮️command/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🔗️causal/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚡️implementations/🦀️rust/📦️lib.rs",
  "compose/client/lib/rs/lib.rs",
  "✏️s/🔌️plugins/🔱️trinity/🎛️apps/✏️rewrite/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🐏️ram/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/📕️norm/🔨️modules/🫀️core/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/📕️norm/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🔨️modules/⚙️engine/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🏛️architect/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🏛️architect/🔨️modules/🦴️spine/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🔨️modules/🔧️op/⚡️implementations/🦀️rust/📦️lib.rs",
];

const CMDENV = [
  "✏️s/🔌️plugins/🏛️architect/🔨️modules/🦴️spine/⚡️implementations/🦀️rust/📦️lib.rs",
  "compose/client/lib/rs/lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🔨️modules/📡️protocol/⚡️implementations/🦀️rust/📦️lib.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🔨️modules/📡️protocol/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🌍️gis/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/➗️mathematical/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🗒️note/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🖨️raster/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/💡️reasoning/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🪐️space/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs",
  "✏️s/🔌️plugins/🌿️vcs/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs",
];

function check(name: string, list: string[]) {
  const seen = new Map<string, string[]>();
  for (const p of list) {
    const key = policyNormalizeRelPath(p);
    if (!seen.has(key)) seen.set(key, []);
    seen.get(key)!.push(p);
  }
  console.log(`\n=== ${name} (${list.length} entries -> ${seen.size} keys) ===`);
  for (const [key, paths] of seen) {
    console.log(`${key}${paths.length > 1 ? "  <<< COLLISION" : ""}`);
    if (paths.length > 1) for (const p of paths) console.log(`    ${p}`);
  }
}

check("DIFF", DIFF);
check("CMDENV", CMDENV);
