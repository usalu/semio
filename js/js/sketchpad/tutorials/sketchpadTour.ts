import { guid } from "../../semio";
import { Tutorial } from "./store";

export const sketchpadTour: Tutorial = {
  id: guid(),
  name: "Sketchpad Tour",
  description: "A comprehensive introduction to Sketchpad - learn to create kits, types, designs, and more.",
  totalDuration: 600,
  icon: "🎓",
  concepts: ["getting-started", "beginner", "introduction"],
  milestones: [
    {
      id: guid(),
      title: "Welcome to Sketchpad",
      description: "Welcome! This tutorial will guide you through the core features of Sketchpad.",
      canSkip: true,
      order: 0,
      duration: 5,
    },
    {
      id: guid(),
      title: "Create a Kit",
      description: "Let''s start by creating a kit. Click the ''+'' button in the home view.",
      commandPattern: { command: "semio.home.kit.create" },
      focusElement: { selector: '[data-panel="home-create-kit"]', highlightMode: "spotlight" },
      canSkip: true,
      order: 1,
      duration: 10,
    },
    {
      id: guid(),
      title: "Open Your Kit",
      description: "Great! Now click on the kit you just created to open it.",
      commandPattern: { command: "semio.home.kit.open" },
      focusElement: { selector: "[data-kit-item]:first-child", highlightMode: "spotlight" },
      canSkip: true,
      order: 2,
      duration: 10,
    },
    {
      id: guid(),
      title: "Create a Type",
      description: "Now let''s create a type. Click the ''+'' button in the types section.",
      commandPattern: { command: "semio.kit.type.create" },
      focusElement: { selector: '[data-panel="kit-create-type"]', highlightMode: "spotlight" },
      canSkip: true,
      order: 3,
      duration: 10,
    },
    {
      id: guid(),
      title: "Tutorial Complete!",
      description: "Congratulations! You''ve completed the Sketchpad tour.",
      canSkip: false,
      order: 4,
      duration: 10,
    },
  ],
};
