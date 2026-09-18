/** ✒️ Boot + interaction-bar probe for the writer react dev playground on port 6062.
 * Usage: cd <repo root> && bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️b1a-writer-boot-probe.mjs"
 */
process.env.SEMIO_B1A_PLUGIN ??= "writer";
process.env.SEMIO_B1A_PORT ??= "6062";
await import("./🐍️b1a-boot-probe.mjs");
