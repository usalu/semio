/** 🏛️ Boot + interaction-bar probe for the architect program react dev playground on port 6090.
 * Usage: cd <repo root> && bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️b1a-architect-boot-probe.mjs"
 */
process.env.SEMIO_B1A_PLUGIN ??= "architect";
process.env.SEMIO_B1A_PORT ??= "6090";
await import("./🐍️b1a-boot-probe.mjs");
