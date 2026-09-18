/** 🎬️ Boot + interaction-bar probe for the sequence react dev playground on port 6077.
 * Usage: cd <repo root> && bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️b1a-sequence-boot-probe.mjs"
 */
process.env.SEMIO_B1A_PLUGIN ??= "sequence";
process.env.SEMIO_B1A_PORT ??= "6077";
await import("./🐍️b1a-boot-probe.mjs");
