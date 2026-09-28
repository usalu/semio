import type { LocalizedLabel, ShellLocale, ShellTerminology } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import type { DslValue, FormQuestion } from "../../../🧬️schema/🧬️mutations/🟦️.ts";

export type ExtensionSurface = { surface: "blueprint" } | { surface: "try"; windowId: string; windowKindId: "forms-try" };

export type ExtensionRenderPayload = ExtensionSurface & {
  fixtureSlug?: string;
  params: DslValue;
  questionId: string;
  controllerId: string;
  interactive: boolean;
}

/** 🎨️ Carries authored routing and exact answers without choosing a provider fixture. */
export function extensionRenderPayload(question: FormQuestion, values: Readonly<Record<string, DslValue>>, controllerId: string, target: ExtensionSurface, interactive: boolean): ExtensionRenderPayload {
  return {
    ...(question.fixtureSlug !== undefined ? { fixtureSlug: question.fixtureSlug } : {}),
    params: structuredClone(Object.hasOwn(values, question.id) ? values[question.id]! : question.params ?? {}),
    questionId: question.id,
    controllerId,
    ...target,
    interactive,
  };
}

export type QuestionKindContribution = {
  appId: string;
  questionKind: string;
  label: LocalizedLabel;
  iconId: string;
  paramsBodyKey: string;
  previewBodyKey: string;
  defaultValueJson?: string;
};

/** 🌐️ Resolves contributed copy using the caller’s explicit language and terminology. */
export function questionKindLabel(contribution: QuestionKindContribution, terminology: ShellTerminology, locale: ShellLocale): string {
  return contribution.label[terminology][locale];
}
