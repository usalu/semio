// usage: node tq.cjs <file> <line> <identifier> : type of the identifier occurrences on that line
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const REPO = "/Users/ueli/Documents/semio";
const cfg = JSON.parse(fs.readFileSync(path.join(REPO, "tsconfig.json"), "utf8"));
const { options } = ts.convertCompilerOptionsFromJson({ ...cfg.compilerOptions, noEmit: true, incremental: false, types: process.env.TS_TYPES ? process.env.TS_TYPES.split(",") : [] }, REPO);
const [file, line, id] = process.argv.slice(2);
const program = ts.createProgram([path.resolve(file)], options);
const sf = program.getSourceFile(path.resolve(file));
const checker = program.getTypeChecker();
const visit = (n) => { if (ts.isIdentifier(n) && n.text === id && sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1 === +line) console.log(checker.typeToString(checker.getTypeAtLocation(n), undefined, ts.TypeFormatFlags.NoTruncation)); ts.forEachChild(n, visit); };
visit(sf);
