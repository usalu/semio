from lib import *
def gittests(t):
    s=read(src("🌿️git/🧪️tests/🟦️.ts"))
    t=sub(t,'import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";','import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,renameSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";')
    t=sub(t,'gitCandidatesV1,gitStateV1,gitViewV1,','gitCandidatesV1,gitGuardsV1,gitPinsHashV1,gitStateV1,gitViewV1,GIT_PINS_V1,GIT_VIEW_PINS_V1,spellingV1,volumeV1,')
    t=sub(t,'expect(await normaliseGitConfigV1(root,"win32")).toEqual(["core.filemode"]);','expect(await normaliseGitConfigV1(root,"win32")).toEqual(["core.filemode","core.longpaths"]);\n expect(value("core.safecrlf")).toBe("false");expect(value("core.eol")).toBe("lf");expect(value("core.protectntfs")).toBe("true");expect(value("core.protecthfs")).toBe("true");')
    t=t.rstrip("\n")+"\n\n"+s[s.index("const putFile="):]
    return t
edit("🌿️git/🧪️tests/🟦️.ts",gittests)
def replicagit(t):
    s=read(src("🔁️replica/🧪️tests/🌿️git.ts"))
    t=sub(t,'import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";','import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";')
    t=sub(t,'import {gitStateV1,INDEX_BASE_V1,','import {gitStateV1,gitViewV1,INDEX_BASE_V1,')
    t=t.rstrip("\n")+"\n\n"+s[s.index("const executableOn="):]
    return t
edit("🔁️replica/🧪️tests/🌿️git.ts",replicagit)
