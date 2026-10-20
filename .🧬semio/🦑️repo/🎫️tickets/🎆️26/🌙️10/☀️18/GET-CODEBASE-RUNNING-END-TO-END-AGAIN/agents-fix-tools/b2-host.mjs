import { edit } from "./lib.mjs";

const D = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/";
export const HOST = { schema: D + "🧬️schema/🔣️.json", decision: D + "📏️decision/🟦️.ts", gate: D + "🎫️gate/🟦️.ts", sampling: D + "📡️sampling/🟦️.ts", tests: D + "🧪️tests/🟦️.ts", samplingTests: D + "🧪️tests/📡️sampling/🟦️.ts" };

export const ERRORS = {
  planRequired: ["Closed host capacity plan required", "Geschlossener Kapazitätsplan des Rechners erforderlich"],
  recordTooLarge: ["Bounded host capacity record required", "Begrenzter Kapazitätseintrag des Rechners erforderlich"],
  grantIdentifierRequired: ["Closed inherited host capacity grant identifier required", "Geschlossene Kennung der geerbten Kapazitätszusage des Rechners erforderlich"],
  queueFull: ["Host capacity queue is full", "Die Warteschlange der Rechnerkapazität ist voll"],
  definitionRequired: ["Closed host capacity {definition} required: {detail}", "Geschlossene Kapazitätsdefinition {definition} des Rechners erforderlich: {detail}"],
  floorOrder: ["Host capacity floor minimum exceeds its maximum", "Die Untergrenze der Rechnerkapazität überschreitet ihre Obergrenze"],
  pressureOrder: ["Host capacity pressure thresholds must rise from warn to critical", "Die Schwellen des Speicherdrucks müssen von Warnung zu kritisch steigen"],
  planJobs: ["Host capacity plan needs jobs between the request bounds", "Der Kapazitätsplan braucht Jobs zwischen den Grenzen der Anfrage"],
  countersUnavailable: ["Linux memory counters unavailable", "Linux-Speicherzähler nicht verfügbar"],
  platformUnsupported: ["Host capacity does not support {platform}", "Die Rechnerkapazität unterstützt {platform} nicht"],
};

const render = (index) => Object.entries(ERRORS).map(([name, texts]) => `            ${JSON.stringify(name)}: ${JSON.stringify(texts[index])}`).join(",\n");

export function build() {
  const H = HOST;
  edit(H.schema, `        "waiter",
        "reasons"
      ],`, `        "waiter",
        "reasons",
        "errors"
      ],`);
  edit(H.schema, `        "waiter": {
          "type": "string"
        }
      }
    },
    "Messages": {`, `        "waiter": {
          "type": "string"
        },
        "errors": {
          "type": "object",
          "additionalProperties": false,
          "required": [
${Object.keys(ERRORS).map((name) => `            ${JSON.stringify(name)}`).join(",\n")}
          ],
          "properties": {
${Object.keys(ERRORS).map((name) => `            ${JSON.stringify(name)}: {\n              "type": "string"\n            }`).join(",\n")}
          }
        }
      }
    },
    "Messages": {`);
  edit(H.schema, `            "queue-order": "an earlier request is still waiting"
          }
        },`, `            "queue-order": "an earlier request is still waiting"
          },
          "errors": {
${render(0).replace(/^ {12}/gm, "            ")}
          }
        },`);
  edit(H.schema, `            "queue-order": "eine frühere Anfrage wartet noch"
          }
        }`, `            "queue-order": "eine frühere Anfrage wartet noch"
          },
          "errors": {
${render(1)}
          }
        }`);

  edit(H.decision, ` if(errors.length)throw Error(\`Closed host capacity \${definition} required: \${errors[0]}\`);`, ` if(errors.length)throw hostCapacityErrorV1("definitionRequired",{definition,detail:errors[0]!});`);
  edit(H.decision, `throw Error("Host capacity floor minimum exceeds its maximum");`, `throw hostCapacityErrorV1("floorOrder");`);
  edit(H.decision, `throw Error("Host capacity pressure thresholds must rise from warn to critical");`, `throw hostCapacityErrorV1("pressureOrder");`);
  edit(H.decision, `throw Error("Host capacity plan needs jobs between the request bounds");`, `throw hostCapacityErrorV1("planJobs");`);
  edit(H.decision, ` return schema.definitions.Messages.const[language][name].replace(/\\{([a-z]+)\\}/gu,(match,key:string)=>key in values?String(values[key]):match);
}`, ` return fillHostCapacityV1(schema.definitions.Messages.const[language][name],values);
}

/** 🧩️ Fills the placeholders of one catalogue text; unknown placeholders stay visible. */
function fillHostCapacityV1(text:string,values:Readonly<Record<string,string|number>>):string{
 return text.replace(/\\{([a-z]+)\\}/gu,(match,key:string)=>key in values?String(values[key]):match);
}

/** 🚨️ Names of the catalogue errors a refused host capacity value, plan or platform raises. */
export type HostCapacityErrorV1=keyof typeof schema.definitions.Messages.const.en.errors;

/** 🖨️ Fills one catalogue error of the chosen language; \`{detail}\` carries the first finding of the schema validator. */
export function formatHostCapacityErrorV1(language:"en"|"de",name:HostCapacityErrorV1,values:Readonly<Record<string,string|number>>={}):string{
 return fillHostCapacityV1(schema.definitions.Messages.const[language].errors[name],values);
}

/** 🚨️ Builds the error of one refusal in the language of the environment, so every diagnostic of the gate speaks English or German like its progress lines. */
export function hostCapacityErrorV1(name:HostCapacityErrorV1,values:Readonly<Record<string,string|number>>={},environment:Readonly<Record<string,string|undefined>>=process.env):Error{
 return Error(formatHostCapacityErrorV1(resolveHostCapacityLanguageV1(environment),name,values));
}`);

  edit(H.gate, `formatHostCapacityV1,isCurrentHostGrantV1,`, `formatHostCapacityV1,hostCapacityErrorV1,isCurrentHostGrantV1,`);
  edit(H.gate, `catch{throw Error("Closed host capacity plan required");}`, `catch{throw hostCapacityErrorV1("planRequired",{},environment);}`);
  edit(H.gate, `throw Error("Bounded host capacity record required");`, `throw hostCapacityErrorV1("recordTooLarge");`);
  edit(H.gate, `throw Error("Closed inherited host capacity grant identifier required");`, `throw Error(schema.definitions.Messages.const[language].errors.grantIdentifierRequired);`);
  edit(H.gate, `throw Error("Host capacity queue is full");`, `throw Error(schema.definitions.Messages.const[language].errors.queueFull);`);

  edit(H.sampling, `import type {HostCapacityPolicyV1,HostSampleV1} from "../📏️decision/🟦️.ts";`, `import {hostCapacityErrorV1,type HostCapacityPolicyV1,type HostSampleV1} from "../📏️decision/🟦️.ts";`);
  edit(H.sampling, `throw Error("Linux memory counters unavailable");`, `throw hostCapacityErrorV1("countersUnavailable");`);
  edit(H.sampling, `throw Error(\`Host capacity does not support \${platform}\`);`, `throw hostCapacityErrorV1("platformUnsupported",{platform});`);

  edit(H.tests, `import {describeHostReasonV1,`, `import {describeHostReasonV1,formatHostCapacityErrorV1,hostCapacityErrorV1,`);
  edit(H.tests, ` expect(resolveHostCapacityLanguageV1({})).toBe("en");
});`, ` expect(resolveHostCapacityLanguageV1({})).toBe("en");
});

test("Every refusal has an English and a German message with the same placeholders and the raised error speaks the chosen language",()=>{
 const english=schema.definitions.Messages.const.en.errors,german=schema.definitions.Messages.const.de.errors,names=Object.keys(english) as (keyof typeof english)[];
 expect(Object.keys(german).sort()).toEqual([...names].sort());
 const placeholders=(text:string)=>[...text.matchAll(/\\{([a-z]+)\\}/gu)].map(match=>match[1]).sort();
 const values={definition:"Grant",detail:"missing id",platform:"plan9"};
 for(const name of names){
  expect(placeholders(german[name]),name).toEqual(placeholders(english[name]));
  for(const language of ["en","de"] as const)expect(formatHostCapacityErrorV1(language,name,values),\`\${language} \${name}\`).not.toMatch(/\\{[a-z]+\\}/u);
 }
 expect(hostCapacityErrorV1("planJobs",{},{SEMIO_LANGUAGE:"de"}).message).toBe(german.planJobs);
 expect(hostCapacityErrorV1("planJobs",{},{SEMIO_LANGUAGE:"en",LANG:"de_DE.UTF-8"}).message).toBe(english.planJobs);
 expect(hostCapacityErrorV1("definitionRequired",{definition:"Grant",detail:"missing id"},{}).message).toBe("Closed host capacity Grant required: missing id");
});`);
  edit(H.samplingTests, `import {readHostCapacityPolicyV1,type HostCommandRunnerV1} from "../../🟦️.ts";`, `import {formatHostCapacityErrorV1,readHostCapacityPolicyV1,resolveHostCapacityLanguageV1,type HostCommandRunnerV1} from "../../🟦️.ts";`);
  edit(H.samplingTests, `.rejects.toThrow("Linux memory counters unavailable");`, `.rejects.toThrow(formatHostCapacityErrorV1(resolveHostCapacityLanguageV1(process.env),"countersUnavailable"));`);
}
