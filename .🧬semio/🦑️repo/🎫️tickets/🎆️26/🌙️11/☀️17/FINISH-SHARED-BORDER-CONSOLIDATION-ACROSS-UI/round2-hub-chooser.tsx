import * as React from "react";
import { createRoot } from "react-dom/client";
import { LanguageChoice } from "@semio-tech/ui-react";
import { probe } from "./border-gap-probe.js";

const OPTIONS = [{ value: "en", endonym: "English" }, { value: "de", endonym: "Deutsch" }] as const;
export const mountChooser = async () => {
  const host = document.createElement("div");
  host.style.cssText = "position:fixed;inset:0;z-index:99999;background:var(--background, #001117)";
  host.className = "h-full w-full";
  document.body.append(host);
  createRoot(host).render(
    <div className="flex h-full w-full items-center justify-center" role="dialog">
      <div className="flex flex-col gap-single">
        <h1 className="text-lg font-semibold">Language · Sprache</h1>
        <LanguageChoice label="Language · Sprache" options={OPTIONS as never} onChoose={() => undefined} />
      </div>
    </div>,
  );
  await new Promise(r => setTimeout(r, 1000));
  const p = probe({ mark: true });
  return { w: innerWidth, e: p.edges, c: p.count, v: p.violations.map(v => v.distance + " " + v.a + " || " + v.b) };
};
