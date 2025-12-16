#!/usr/bin/env npx tsx



import { InvalidKit } from "@semio/assets";
import * as fs from "fs";
import * as path from "path";
import { Kit, serializeValidationResult, validateSemioKit } from "../js/js/semio";

const main = () => {
    const kit = InvalidKit as unknown as Kit;
    const result = validateSemioKit(kit);
    const json = serializeValidationResult(result);

    const outputPath = path.join(__dirname, "..", "assets", "semio", "validation.json");
    fs.writeFileSync(outputPath, json + "\n");

    console.log(`Generated ${outputPath}`);
    console.log(`Found ${result.issues.length} validation issues`);
};

main();
