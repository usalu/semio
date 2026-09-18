/** 🌿️ Boot + interaction-bar probe for the vcs react dev playground on port 6075.
 * Usage: cd <repo root> && bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️b1a-vcs-boot-probe.mjs"
 */
process.env.SEMIO_B1A_PLUGIN ??= "vcs";
process.env.SEMIO_B1A_PORT ??= "6075";
await import("./🐍️b1a-boot-probe.mjs");
