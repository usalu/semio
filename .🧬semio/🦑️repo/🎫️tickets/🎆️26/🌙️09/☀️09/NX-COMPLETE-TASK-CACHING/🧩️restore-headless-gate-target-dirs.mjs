const files = [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"];
const names = ["⚖️gate📇️native-openable-catalog-provider🌎️hub", "⚖️gate📇️headless-native-catalog🗄️stdio", "⚖️gate📇️native-catalog-selection🌎️hub"];
for (const file of files) {
  const lines = (await Bun.file(file).text()).split("\n");
  for (const name of names) {
    const start = lines.findIndex((line) => line.includes(`"name": "${name}"`));
    if (start < 0) throw new Error(`${file}: missing ${name}`);
    const end = lines.findIndex((line, index) => index > start && /^    \},?$/.test(line));
    const body = lines.slice(start, end);
    if (body.some((line) => line.includes('"CARGO_TARGET_DIR"'))) continue;
    const offset = body.findIndex((line) => line.includes('"SEMIO_TEST_ARTIFACT_DIR"'));
    const match = body[offset]?.match(/^(\s*)"SEMIO_TEST_ARTIFACT_DIR": "([^"]+)",$/u);
    if (!match) throw new Error(`${file}: ${name} has no comma-terminated SEMIO_TEST_ARTIFACT_DIR`);
    lines.splice(start + offset + 1, 0, `${match[1]}"CARGO_TARGET_DIR": "${match[2]}/cargo-target",`);
    console.log(`${file}: restored ${name}`);
  }
  await Bun.write(file, lines.join("\n"));
}
