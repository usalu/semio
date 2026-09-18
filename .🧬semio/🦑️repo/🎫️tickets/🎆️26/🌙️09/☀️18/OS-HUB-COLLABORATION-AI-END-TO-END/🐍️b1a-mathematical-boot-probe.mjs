/** ➗️ Boot + interaction-bar probe for the mathematical equation react dev playground on port 6084.
 * Usage: cd <repo root> && bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️b1a-mathematical-boot-probe.mjs"
 */
process.env.SEMIO_B1A_PLUGIN ??= "mathematical";
process.env.SEMIO_B1A_PORT ??= "6084";
await import("./🐍️b1a-boot-probe.mjs");
