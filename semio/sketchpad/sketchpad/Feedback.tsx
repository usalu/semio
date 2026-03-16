// #region Header

// js/semio/sketchpad/Feedback.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// #region Imports

import { CheckIcon, ChatIcon as FeedbackIcon } from "@semio/assets";
import { FC, useCallback, useLayoutEffect, useMemo } from "react";
import { useLabel } from "../i18n";
import { Button, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Textarea, ToolbarGroup, useXStateSelector as useSelector, useTranslation, useNavigate } from "../../../semio-elements/ui";
import type { AppConfig, AppPlugin, HookResult, PanelDefinition } from "./shared";
import { conditionalHookResult, createPanelDefinition, EMPTY_PANEL_VISIBILITY, PanelKind, registerAppPlugin, registerEventHandler } from "./shared";
import { Canvas, FeedbackAppKind, FeedbackAppState, FeedbackFormData, FeedbackKind, useAddPanelSection, useAppType, useRemovePanelSection, useSketchpadActor } from "./Sketchpad";

// #endregion Imports

// #region Feedback App Plugin Registration

// [👤semio📚js🗃️sketchpad💻feedbacktsx🔖feedbackapppluginregistration](semiorepo://section/SEMIO/JS/SKETCHPAD/FEEDBACK.TSX/FEEDBACK-APP-PLUGIN-REGISTRATION)
// MUST register the Feedback app plugin with default state and event handlers.

// [👤semio📚js🗃️sketchpad💻feedback🔖feedbackapppluginregistration🪨createdefaultfeedbackstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Feedback%20App%20Plugin%20Registration/d/i/createDefaultFeedbackState)
/**
 * [👤semio📚js🗃️sketchpad💻feedback🔖feedbackapppluginregistration🪨createdefaultfeedbackstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Feedback%20App%20Plugin%20Registration/d/i/createDefaultFeedbackState)
 * createDefaultFeedbackState holds the data fields for a createDefaultFeedbackState record.
 **/
const createDefaultFeedbackState = (): FeedbackAppState => ({
  panelVisibility: { ...EMPTY_PANEL_VISIBILITY },
  formData: {
    kind: "bug",
    title: "",
    description: "",
    app: undefined,
    name: undefined,
    email: undefined,
  },
  isSubmitting: false,
  isSubmitted: false,
  error: undefined,
});

/**
 * [👤semio📚js🗃️sketchpad💻feedback🔖feedbackapppluginregistration🪨feedbackappplugin](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Feedback%20App%20Plugin%20Registration/d/i/feedbackAppPlugin)
 * feedbackAppPlugin holds the data fields for a feedbackAppPlugin record.
 **/
const feedbackAppPlugin: AppPlugin = {
  id: "feedback",
  namespace: "FEEDBACK",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: createDefaultFeedbackState,
  },
};

if (typeof window !== "undefined") {
  registerAppPlugin(feedbackAppPlugin);

  registerEventHandler("FEEDBACK.TOGGLE_PANEL", {
    action: (context: any, event: any) => ({
      feedbackApp: {
        ...context.feedbackApp,
        panelVisibility: {
          ...context.feedbackApp.panelVisibility,
          [event.panel]: !context.feedbackApp.panelVisibility[event.panel],
        },
      },
    }),
  });

  registerEventHandler("FEEDBACK.SET_FORM_DATA", {
    action: (context: any, event: any) => ({
      feedbackApp: {
        ...context.feedbackApp,
        formData: { ...context.feedbackApp.formData, ...event.data },
      },
    }),
  });

  registerEventHandler("FEEDBACK.RESET_FORM", {
    action: (context: any) => ({
      feedbackApp: {
        ...context.feedbackApp,
        formData: {
          kind: "bug",
          title: "",
          description: "",
          app: undefined,
          name: undefined,
          email: undefined,
        },
        isSubmitting: false,
        isSubmitted: false,
        error: undefined,
      },
    }),
  });

  registerEventHandler("FEEDBACK.SET_SUBMITTING", {
    action: (context: any, event: any) => ({
      feedbackApp: { ...context.feedbackApp, isSubmitting: event.isSubmitting },
    }),
  });

  registerEventHandler("FEEDBACK.SET_SUBMITTED", {
    action: (context: any, event: any) => ({
      feedbackApp: { ...context.feedbackApp, isSubmitted: event.isSubmitted, isSubmitting: false },
    }),
  });

  registerEventHandler("FEEDBACK.SET_ERROR", {
    action: (context: any, event: any) => ({
      feedbackApp: { ...context.feedbackApp, error: event.error, isSubmitting: false },
    }),
  });
}

// #endregion Feedback App Plugin Registration

// #region Triadic Hooks

// [👤semio📚js🗃️sketchpad💻feedbacktsx🔖triadichooks](semiorepo://section/SEMIO/JS/SKETCHPAD/FEEDBACK.TSX/TRIADIC-HOOKS)
// MUST provide triadic hooks for accessing and mutating Feedback app state.

/**
 * [👤semio📚js🗃️sketchpad💻feedback🔖triadichooks🪨defaultformdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Triadic%20Hooks/d/i/DEFAULT_FORM_DATA)
 * DEFAULT_FORM_DATA holds the data fields for a DEFAULT_FORM_DATA record.
 **/
const DEFAULT_FORM_DATA: FeedbackFormData = {
  kind: "bug",
  title: "",
  description: "",
  app: undefined,
  name: undefined,
  email: undefined,
};

/**
 * Triadic hook for feedback form data state.
 *MUST return current form data, setter, and writability flag.
 * [👤semio📚js🗃️sketchpad💻feedback🔖triadichooks🛠️usefeedbackformdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Triadic%20Hooks/d/i/useFeedbackFormData)
 **/
export function useFeedbackFormData(): HookResult<FeedbackFormData> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => snapshot.context.feedbackApp?.formData ?? DEFAULT_FORM_DATA);
  const canSetEvent = useMemo(() => ({ type: "FEEDBACK.SET_FORM_DATA" as const, data: {} as Partial<FeedbackFormData> }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (data: FeedbackFormData) => {
      actor.send({ type: "FEEDBACK.SET_FORM_DATA", data });
    };
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

/**
 * Triadic hook for feedback submission loading state.
 *MUST return current submitting flag, setter, and writability flag.
 * [👤semio📚js🗃️sketchpad💻feedback🔖triadichooks🛠️usefeedbackissubmitting](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Triadic%20Hooks/d/i/useFeedbackIsSubmitting)
 **/
export function useFeedbackIsSubmitting(): HookResult<boolean> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => snapshot.context.feedbackApp?.isSubmitting ?? false);
  const canSetEvent = useMemo(() => ({ type: "FEEDBACK.SET_SUBMITTING" as const, isSubmitting: false }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (isSubmitting: boolean) => {
      actor.send({ type: "FEEDBACK.SET_SUBMITTING", isSubmitting });
    };
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

/**
 * Triadic hook for feedback submission completion state.
 *MUST return current submitted flag, setter, and writability flag.
 * [👤semio📚js🗃️sketchpad💻feedback🔖triadichooks🛠️usefeedbackissubmitted](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Triadic%20Hooks/d/i/useFeedbackIsSubmitted)
 **/
export function useFeedbackIsSubmitted(): HookResult<boolean> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => snapshot.context.feedbackApp?.isSubmitted ?? false);
  const canSetEvent = useMemo(() => ({ type: "FEEDBACK.SET_SUBMITTED" as const, isSubmitted: false }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (isSubmitted: boolean) => {
      actor.send({ type: "FEEDBACK.SET_SUBMITTED", isSubmitted });
    };
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

/**
 * Triadic hook for feedback error state.
 *MUST return current error message, setter, and writability flag.
 * [👤semio📚js🗃️sketchpad💻feedback🔖triadichooks🛠️usefeedbackerror](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Triadic%20Hooks/d/i/useFeedbackError)
 **/
export function useFeedbackError(): HookResult<string | undefined> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => snapshot.context.feedbackApp?.error);
  const canSetEvent = useMemo(() => ({ type: "FEEDBACK.SET_ERROR" as const, error: "" }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (error: string | undefined) => {
      actor.send({ type: "FEEDBACK.SET_ERROR", error });
    };
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

/**
 * Triadic hook for resetting the feedback form to defaults.
 *MUST return reset callback and availability flag.
 * [👤semio📚js🗃️sketchpad💻feedback🔖triadichooks🔖app🛠️feedbacktoolbar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Triadic%20Hooks/s/App/d/i/FeedbackToolbar)
 **/
export function useFeedbackReset(): [(() => void) | undefined, boolean] {
  const actor = useSketchpadActor();
  const canResetEvent = useMemo(() => ({ type: "FEEDBACK.RESET_FORM" as const }), []);
  const canReset = useSelector(actor, (snapshot) => snapshot.can(canResetEvent));
  const reset = useMemo(() => {
    if (!canReset) return undefined;
    return () => {
      actor.send({ type: "FEEDBACK.RESET_FORM" });
    };
  }, [actor, canReset]);
  return [reset, canReset];
}

// #endregion Triadic Hooks

// #region Components

// #region Form

// [👤semio📚js🗃️sketchpad💻feedbacktsx🔖components🔖form](semiorepo://section/SEMIO/JS/SKETCHPAD/FEEDBACK.TSX/COMPONENTS/FORM)
// MUST render feedback form for submitting bug reports and ideas.

// [👤semio📚js🗃️sketchpad💻feedback🔖components🔖form🪨feedbackform](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Components/s/Form/d/i/FeedbackForm)
/**
 * [👤semio📚js🗃️sketchpad💻feedback🔖components🔖form🪨feedbackform](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Components/s/Form/d/i/FeedbackForm)
 * FeedbackForm holds the data fields for a FeedbackForm record.
 **/
const FeedbackForm: FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [formData, setFormData, canSetFormData] = useFeedbackFormData();
  const [isSubmitting, setIsSubmitting, canSetIsSubmitting] = useFeedbackIsSubmitting();
  const [isSubmitted, setIsSubmitted, canSetIsSubmitted] = useFeedbackIsSubmitted();
  const [error, setError, canSetError] = useFeedbackError();
  const [reset, canReset] = useFeedbackReset();

  const kind = formData.kind;
  const title = formData.title;
  const description = formData.description;
  const app = formData.app;
  const name = formData.name ?? "";
  const email = formData.email ?? "";

  const setKind = useCallback((kind: FeedbackKind) => setFormData?.({ ...formData, kind }), [formData, setFormData]);
  const setTitle = useCallback((title: string) => setFormData?.({ ...formData, title }), [formData, setFormData]);
  const setDescription = useCallback((description: string) => setFormData?.({ ...formData, description }), [formData, setFormData]);
  const setApp = useCallback((app: FeedbackAppKind | undefined) => setFormData?.({ ...formData, app }), [formData, setFormData]);
  const setName = useCallback((name: string) => setFormData?.({ ...formData, name: name || undefined }), [formData, setFormData]);
  const setEmail = useCallback((email: string) => setFormData?.({ ...formData, email: email || undefined }), [formData, setFormData]);

  const kindLabel = useLabel("semio.sketchpad.app.feedback.form.kind");
  const titleLabel = useLabel("semio.sketchpad.app.feedback.form.title");
  const descriptionLabel = useLabel("semio.sketchpad.app.feedback.form.description");
  const appLabel = useLabel("semio.sketchpad.app.feedback.form.app");
  const nameLabel = useLabel("semio.sketchpad.app.feedback.form.name");
  const emailLabel = useLabel("semio.sketchpad.app.feedback.form.email");
  const submitLabel = useLabel("semio.sketchpad.app.feedback.form.submit");
  const sendAnotherLabel = useLabel("semio.sketchpad.app.feedback.success.sendAnother");
  const goHomeLabel = useLabel("semio.sketchpad.app.feedback.success.goHome");
  const thankYouLabel = useLabel("semio.sketchpad.app.feedback.success.thankYou");
  const bugLabel = useLabel("semio.sketchpad.app.feedback.kind.bug");
  const ideaLabel = useLabel("semio.sketchpad.app.feedback.kind.idea");

  const appOptions: { value: FeedbackAppKind; label: string }[] = [
    { value: "home", label: t("semio.sketchpad.app.feedback.appOption.home.label.normal", "Home") },
    { value: "kit", label: t("semio.sketchpad.app.feedback.appOption.kit.label.normal", "Kit") },
    { value: "design", label: t("semio.sketchpad.app.feedback.appOption.design.label.normal", "Design") },
    { value: "type", label: t("semio.sketchpad.app.feedback.appOption.type.label.normal", "Type") },
    { value: "quality", label: t("semio.sketchpad.app.feedback.appOption.quality.label.normal", "Quality") },
    { value: "docs", label: t("semio.sketchpad.app.feedback.appOption.docs.label.normal", "Docs") },
    { value: "feedback", label: t("semio.sketchpad.app.feedback.appOption.feedback.label.normal", "Feedback") },
  ];

  const handleSubmit = useCallback(async () => {
    if (!title.trim()) {
      setError?.(t("semio.sketchpad.app.feedback.error.titleRequired.label.normal", "Title is required"));
      return;
    }
    if (!description.trim()) {
      setError?.(t("semio.sketchpad.app.feedback.error.descriptionRequired.label.normal", "Description is required"));
      return;
    }
    if (kind === "bug" && !app) {
      setError?.(t("semio.sketchpad.app.feedback.error.appRequired.label.normal", "Please select which app the bug occurred in"));
      return;
    }

    setIsSubmitting?.(true);
    setError?.(undefined);

    try {
      const payload: FeedbackFormData = {
        kind,
        title: title.trim(),
        description: description.trim(),
        app: kind === "bug" ? app : undefined,
        name: name.trim() || undefined,
        email: email.trim() || undefined,
      };

      const response = await fetch("https://api.semio.tech/feedback", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) throw new Error("Failed to submit feedback");
      setIsSubmitted?.(true);
    } catch {
      setError?.(t("semio.sketchpad.app.feedback.error.submitFailed.label.normal", "Failed to submit feedback. Please try again."));
    } finally {
      setIsSubmitting?.(false);
    }
  }, [kind, title, description, app, name, email, t, setError, setIsSubmitting, setIsSubmitted]);

  const handleReset = useCallback(() => {
    reset?.();
  }, [reset]);

  const handleGoHome = useCallback(() => {
    navigate("/");
  }, [navigate]);

  if (isSubmitted) {
    return (
      <div className="flex flex-col items-center justify-center gap-4 p-8 max-w-md mx-auto">
        <div className="text-4xl">🎉</div>
        <h2 id="semio.sketchpad.app.feedback.success.thankYou" className="text-xl font-semibold text-center">
          {thankYouLabel}
        </h2>
        <p className="text-center text-muted-foreground">{t("semio.sketchpad.app.feedback.success.message.label.normal", "Your feedback has been received. We appreciate your contribution!")}</p>
        <div className="flex gap-2 mt-4">
          <Button id="semio.sketchpad.app.feedback.success.sendAnother" onClick={handleReset} variant="outline">
            {sendAnotherLabel}
          </Button>
          <Button id="semio.sketchpad.app.feedback.success.goHome" onClick={handleGoHome}>
            {goHomeLabel}
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4 p-8 max-w-md mx-auto">
      <h1 className="text-2xl font-bold">{t("semio.sketchpad.app.feedback.title.label.normal", "Feedback")}</h1>
      <p className="text-muted-foreground">{t("semio.sketchpad.app.feedback.subtitle.label.normal", "Help us improve semio by reporting bugs or sharing ideas.")}</p>

      <div className="flex flex-col gap-1">
        <label htmlFor="semio.sketchpad.app.feedback.form.kind" className="text-sm font-medium">
          {kindLabel}
        </label>
        <Select id="semio.sketchpad.app.feedback.form.kind.select" value={kind} onValueChange={(v) => setKind(v as FeedbackKind)}>
          <SelectTrigger id="semio.sketchpad.app.feedback.form.kind">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem id="semio.sketchpad.app.feedback.kind.bug" value="bug">
              🐛 {bugLabel}
            </SelectItem>
            <SelectItem id="semio.sketchpad.app.feedback.kind.idea" value="idea">
              💡 {ideaLabel}
            </SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div className="flex flex-col gap-1">
        <label htmlFor="semio.sketchpad.app.feedback.form.title" className="text-sm font-medium">
          {titleLabel}
        </label>
        <Input id="semio.sketchpad.app.feedback.form.title" value={title} onChange={(e) => setTitle(e.target.value)} placeholder={t("semio.sketchpad.app.feedback.form.titlePlaceholder.label.normal", "Enter a brief title...")} />
      </div>

      {kind === "bug" && (
        <div className="flex flex-col gap-1">
          <label htmlFor="semio.sketchpad.app.feedback.form.app" className="text-sm font-medium">
            {appLabel}
          </label>
          <Select id="semio.sketchpad.app.feedback.form.app.select" value={app || ""} onValueChange={(v) => setApp(v as FeedbackAppKind)}>
            <SelectTrigger id="semio.sketchpad.app.feedback.form.app">
              <SelectValue placeholder={t("semio.sketchpad.app.feedback.form.appPlaceholder.label.normal", "Select app...")} />
            </SelectTrigger>
            <SelectContent>
              {appOptions.map((option) => (
                <SelectItem key={option.value} id={`semio.sketchpad.app.feedback.appOption.${option.value}`} value={option.value}>
                  {option.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      )}

      <div className="flex flex-col gap-1">
        <label htmlFor="semio.sketchpad.app.feedback.form.description" className="text-sm font-medium">
          {descriptionLabel}
        </label>
        <Textarea
          id="semio.sketchpad.app.feedback.form.description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder={
            kind === "bug" ? t("semio.sketchpad.app.feedback.form.bugDescriptionPlaceholder.label.normal", "Describe what happened...") : t("semio.sketchpad.app.feedback.form.ideaDescriptionPlaceholder.label.normal", "Describe your idea...")
          }
          className="min-h-[120px]"
        />
      </div>

      <div className="border-t border-element pt-4 mt-2">
        <p className="text-sm text-muted-foreground mb-4">{t("semio.sketchpad.app.feedback.optional.label.normal", "Optional contact information")}</p>

        <div className="flex flex-col gap-4">
          <div className="flex flex-col gap-1">
            <label htmlFor="semio.sketchpad.app.feedback.form.name" className="text-sm font-medium">
              {nameLabel}
            </label>
            <Input id="semio.sketchpad.app.feedback.form.name" value={name} onChange={(e) => setName(e.target.value)} placeholder={t("semio.sketchpad.app.feedback.form.namePlaceholder.label.normal", "Your name (optional)")} />
          </div>

          <div className="flex flex-col gap-1">
            <label htmlFor="semio.sketchpad.app.feedback.form.email" className="text-sm font-medium">
              {emailLabel}
            </label>
            <Input id="semio.sketchpad.app.feedback.form.email" type="email" value={email} onChange={(e) => setEmail(e.target.value)} placeholder={t("semio.sketchpad.app.feedback.form.emailPlaceholder.label.normal", "your@email.com (optional)")} />
          </div>
        </div>
      </div>

      {error && <div className="text-destructive text-sm p-2 bg-destructive/10 rounded">{error}</div>}

      <Button id="semio.sketchpad.app.feedback.form.submit" onClick={handleSubmit} disabled={isSubmitting} className="mt-4">
        {isSubmitting ? t("semio.sketchpad.app.feedback.form.submitting.label.normal", "Submitting...") : submitLabel}
      </Button>
    </div>
  );
};

// #endregion Form

// #endregion Components

// #region App

// [👤semio📚js🗃️sketchpad💻feedbacktsx🔖app](semiorepo://section/SEMIO/JS/SKETCHPAD/FEEDBACK.TSX/APP)
// MUST integrate feedback app with toolbar and layout canvas.

// [👤semio📚js🗃️sketchpad💻feedback🔖app🪨feedbacktoolbar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/App/d/i/FeedbackToolbar)
/**
 * [👤semio📚js🗃️sketchpad💻feedback🔖app🪨feedbacktoolbar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/App/d/i/FeedbackToolbar)
 * FeedbackToolbar holds the data fields for a FeedbackToolbar record.
 **/
const FeedbackToolbar: FC = () => {
  const { t } = useTranslation();
  const submitLabel = useLabel("semio.sketchpad.app.feedback.form.submit");

  const handleSendClick = () => {
    const submitButton = document.getElementById("semio.sketchpad.app.feedback.form.submit") as HTMLButtonElement;
    if (submitButton) {
      submitButton.click();
    }
  };

  return (
    <ToolbarGroup>
      <Button id="semio.sketchpad.app.feedback.toolbar.send" onClick={handleSendClick} className="gap-single">
        <CheckIcon className="size-small" />
        {submitLabel}
      </Button>
    </ToolbarGroup>
  );
};

// [👤semio📚js🗃️sketchpad💻feedback🔖app🪨feedback](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/App/d/i/Feedback)
/**
 * [👤semio📚js🗃️sketchpad💻feedback🔖app🪨feedback](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/App/d/i/Feedback)
 * Feedback holds the data fields for a Feedback record.
 **/
const Feedback: FC = () => {
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useLayoutEffect(() => {
    if (appType !== "feedback") return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.feedback.toolbar.send",
      specificity: 20,
      order: 0,
      toolbarGroup: {
        id: "actions",
        labelId: "semio.sketchpad.toolbar.parent.actions",
        order: 50,
      },
      content: <FeedbackToolbar />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.feedback.toolbar.send");
    };
  }, [appType, addSection, removeSection]);

  return (
    <Canvas>
      <FeedbackForm />
    </Canvas>
  );
};

export default Feedback;

// #endregion App

// #region Config

// [👤semio📚js🗃️sketchpad💻feedbacktsx🔖config](semiorepo://section/SEMIO/JS/SKETCHPAD/FEEDBACK.TSX/CONFIG)
// MUST define app configuration for the Feedback app.

/**
 * Feedback app configuration with routing, component, and panel definitions.
 * [👤semio📚js🗃️sketchpad💻feedback🔖config🪨config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Config/d/i/config)
 **/
export const config = {
  id: "feedback",
  component: Feedback,
  routeSegments: [{ path: "feedback" }],
  getPanels: (): PanelDefinition[] => [createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show")],
  matchesPath: (pathParts) => pathParts.length === 1 && pathParts[0] === "feedback",
  order: 10,
};

// #endregion Config

// #region Global Footer Item

// [👤semio📚js🗃️sketchpad💻feedbacktsx🔖globalfooteritem](semiorepo://section/SEMIO/JS/SKETCHPAD/FEEDBACK.TSX/GLOBAL-FOOTER-ITEM)
// MUST re-export the feedback icon for the footer item.

export { FeedbackIcon };

// #endregion Global Footer Item
