/** 🎞️ Boot + interaction-bar probe for the animate presentation react dev playground on port 6051.
 * Usage: cd <repo root> && bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️b1a-animate-boot-probe.mjs"
 */
process.env.SEMIO_B1A_PLUGIN ??= "animate";
process.env.SEMIO_B1A_PORT ??= "6051";
await import("./🐍️b1a-boot-probe.mjs");
