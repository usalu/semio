#!/usr/bin/env python3
"""Writes the NEW files of the envleak batch into a work root (a mirror): `build_module.py <work-root>`."""
import json, os, sys

BZ = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
root = os.path.abspath(sys.argv[1])

def put(rel, text):
    path = os.path.join(root, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf8").write(text)

schema = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "Bazel keyed command environment",
    "description": "🌿️ Schema-first contract of the environment a command runs in when its output keys a Bazel action or a reusable plan (a `runtime` input row of the task graph). The digest of such a command is the digest of its STANDARD OUTPUT only: standard error is diagnostics and never keys anything. The command receives the issued task environment minus (1) every diagnostic switch listed in `Policy.diagnostic`, which changes what a process prints (the phase profile prints a JSON document with pid and milliseconds, the traces print time stamps) and (2) every ambient host, agent or session variable of the world witness policy (`ambientEnvironment` in the witness schema). / Schema-first Vertrag der Umgebung, in der ein Befehl laeuft, dessen Ausgabe eine Bazel-Aktion oder einen wiederverwendbaren Plan schluesselt (eine `runtime`-Eingabezeile des Aufgabengraphen). Der Digest eines solchen Befehls ist der Digest seiner STANDARDAUSGABE allein: die Fehlerausgabe ist Diagnose und schluesselt nie etwas. Der Befehl erhaelt die ausgestellte Aufgabenumgebung ohne (1) jeden Diagnoseschalter aus `Policy.diagnostic`, der veraendert, was ein Prozess ausgibt (das Phasenprofil gibt ein JSON-Dokument mit Pid und Millisekunden aus, die Traces geben Zeitstempel aus) und (2) jede umgebende Host-, Agenten- oder Sitzungsvariable der Weltzeugen-Richtlinie (`ambientEnvironment` im Zeugen-Schema).",
    "definitions": {
        "Policy": {
            "type": "object",
            "description": "Authored values. `diagnostic`: names of switches that change what a process prints and therefore never reach a keyed command. `runtime`: the finite bounds of the captured standard output.",
            "const": {
                "version": 1,
                "diagnostic": ["SEMIO_REPO_PROFILE", "SEMIO_LEASE_TRACE", "SEMIO_PROGRESS_DEADLINE_TRACE", "NODE_DEBUG", "BUN_CONFIG_VERBOSE_FETCH"],
                "runtime": {"maximumBytes": 4194304, "maximumLines": 65536},
            },
        },
        "Messages": {
            "type": "object",
            "description": "Refusal text in English and German; `{name}` placeholders are replaced by the reported values.",
            "const": {
                "en": {"failed": "Runtime input {runtime} exited with status {status} and signal {signal}: {stderr}"},
                "de": {"failed": "Laufzeiteingabe {runtime} endete mit Status {status} und Signal {signal}: {stderr}"},
            },
        },
    },
}
put(f"{BZ}/🌿️environment/🧬️schema/🔣️.json", json.dumps(schema, ensure_ascii=False, indent=2) + "\n")

module = '''import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {captureOwnedCommandResultV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {isAmbientEnvironmentNameV1} from "../../../🔭️witness/🟨️.mjs";

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const,diagnostic=new Set<string>(policy.diagnostic);

/** 🌿️ The authored policy of the keyed command environment. */
export const BAZEL_KEYED_COMMAND_POLICY_V1=policy;
/** 🧾️ The environment of a process as the build issues it. */
export type BazelCommandEnvironmentV1=Readonly<Record<string,string|undefined>>;
/** 📬️ What a finished command hands back, separated by stream. */
export interface BazelCommandCaptureV1{readonly stdout:string;readonly stderr:string;readonly status:number|null;readonly signal:string|null}
/** 🔌️ The way a command is run and captured; the owned separated capture by default, replaceable by a test. */
export type BazelCommandCapturerV1=(command:string,args:readonly string[],cwd:string,options:{readonly control:ResourceLeaseControl;readonly env:BazelCommandEnvironmentV1;readonly output:{readonly maximumBytes:number;readonly maximumLines:number}})=>Promise<BazelCommandCaptureV1>;

/** 🧼️ The environment a keyed command receives: the issued one without any diagnostic switch and without any ambient host, agent or session variable, so that a switch or a shell never changes what the command prints. */
export function bazelKeyedCommandEnvironmentV1(environment:BazelCommandEnvironmentV1):Record<string,string|undefined>{
 return Object.fromEntries(Object.entries(environment).filter(([name])=>!diagnostic.has(name)&&!isAmbientEnvironmentNameV1(name)));
}

/** 🏃️ Runs one `runtime` input row and returns its standard output, which alone keys the action or the plan; a failed command is refused in both languages with its standard error as the explanation. */
export async function runBazelRuntimeCommandV1(root:string,runtime:string,environment:BazelCommandEnvironmentV1,control:ResourceLeaseControl,capture:BazelCommandCapturerV1=captureOwnedCommandResultV1):Promise<string>{
 const result=await capture(process.execPath,["exec",runtime],root,{control,env:bazelKeyedCommandEnvironmentV1(environment),output:policy.runtime});
 if(result.status===0&&result.signal===null)return result.stdout;
 const say=(table:typeof messages.en)=>table.failed.replace("{runtime}",runtime).replace("{status}",String(result.status)).replace("{signal}",String(result.signal)).replace("{stderr}",result.stderr.slice(-2048));
 throw new Error(say(messages.en)+" / "+say(messages.de));
}
'''
put(f"{BZ}/🌿️environment/🟦️.ts", module)
