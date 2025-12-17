// #region Header

// Feedback.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

// #region Imports

import { CheckIcon, ChatIcon as FeedbackIcon } from "@semio/assets";
import { FC, useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { useLabel } from "../i18n";
import { Button, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Textarea, Window } from "./elements";
import type { AppConfig, AppPlugin, PanelDefinition, PanelVisibility } from "./shared";
import { createPanelDefinition, PanelKind, registerAppPlugin, registerEventHandler } from "./shared";
import { Canvas, useAddPanelSection, useAppType, useRemovePanelSection } from "./Sketchpad";

// #endregion Imports

// #region Types

export type FeedbackKind = "bug" | "idea";

export type FeedbackAppKind = "home" | "kit" | "design" | "type" | "quality" | "docs" | "feedback";

export interface FeedbackFormData {
  kind: FeedbackKind;
  title: string;
  description: string;
  app?: FeedbackAppKind;
  name?: string;
  email?: string;
}

export interface FeedbackState {
  panelVisibility: PanelVisibility;
  formData: FeedbackFormData;
  isSubmitting: boolean;
  isSubmitted: boolean;
  error?: string;
}

export interface FeedbackDiff {
  panelVisibility?: Partial<PanelVisibility>;
  formData?: Partial<FeedbackFormData>;
  isSubmitting?: boolean;
  isSubmitted?: boolean;
  error?: string;
}

// #endregion Types

// #region Feedback App Plugin Registration

const feedbackAppPlugin: AppPlugin = {
  id: "feedback",
  namespace: "FEEDBACK",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: (): FeedbackState => ({
      panelVisibility: { toolbar: true, workbench: false, details: false, chat: false, settings: false },
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
    }),
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

// #region Components

// #region Form

const FeedbackForm: FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [kind, setKind] = useState<FeedbackKind>("bug");
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [app, setApp] = useState<FeedbackAppKind | undefined>(undefined);
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSubmitted, setIsSubmitted] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);

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
      setError(t("semio.sketchpad.app.feedback.error.titleRequired.label.normal", "Title is required"));
      return;
    }
    if (!description.trim()) {
      setError(t("semio.sketchpad.app.feedback.error.descriptionRequired.label.normal", "Description is required"));
      return;
    }
    if (kind === "bug" && !app) {
      setError(t("semio.sketchpad.app.feedback.error.appRequired.label.normal", "Please select which app the bug occurred in"));
      return;
    }

    setIsSubmitting(true);
    setError(undefined);

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
      setIsSubmitted(true);
    } catch {
      setError(t("semio.sketchpad.app.feedback.error.submitFailed.label.normal", "Failed to submit feedback. Please try again."));
    } finally {
      setIsSubmitting(false);
    }
  }, [kind, title, description, app, name, email, t]);

  const handleReset = useCallback(() => {
    setKind("bug");
    setTitle("");
    setDescription("");
    setApp(undefined);
    setName("");
    setEmail("");
    setIsSubmitting(false);
    setIsSubmitted(false);
    setError(undefined);
  }, []);

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
      <p className="text-muted-foreground">{t("semio.sketchpad.app.feedback.subtitle.label.normal", "Help us improve Semio by reporting bugs or sharing ideas.")}</p>

      <div className="flex flex-col gap-1">
        <label htmlFor="semio.sketchpad.app.feedback.form.kind" className="text-sm font-medium">
          {kindLabel}
        </label>
        <Select id="semio.sketchpad.app.feedback.form.kind" value={kind} onValueChange={(v) => setKind(v as FeedbackKind)}>
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
          <Select id="semio.sketchpad.app.feedback.form.app" value={app || ""} onValueChange={(v) => setApp(v as FeedbackAppKind)}>
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

      <div className="border-t pt-4 mt-2">
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
    <div className="flex items-center gap-single">
      <Button id="semio.sketchpad.app.feedback.toolbar.send" onClick={handleSendClick} className="gap-single">
        <CheckIcon className="size-small" />
        {submitLabel}
      </Button>
    </div>
  );
};

const Feedback: FC = () => {
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useEffect(() => {
    if (appType !== "feedback") return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.feedback.toolbar.send",
      specificity: 20,
      order: 0,
      content: <FeedbackToolbar />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.feedback.toolbar.send");
    };
  }, [appType, addSection, removeSection]);

  return (
    <Canvas>
      <Window id="feedback-form" className="h-full w-full overflow-auto">
        <FeedbackForm />
      </Window>
    </Canvas>
  );
};

export default Feedback;

// #endregion App

// #region Config

export const config: AppConfig = {
  id: "feedback",
  component: Feedback,
  routeSegments: [{ path: "feedback" }],
  getPanels: (): PanelDefinition[] => [createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show")],
  matchesPath: (pathParts) => pathParts.length === 1 && pathParts[0] === "feedback",
  order: 10,
};

// #endregion Config

// #region Global Footer Item

export { FeedbackIcon };

// #endregion Global Footer Item
