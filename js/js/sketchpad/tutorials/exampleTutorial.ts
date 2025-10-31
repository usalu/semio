// #region Header

// exampleTutorial.ts

// Example tutorial demonstrating the tutorial system

// #endregion

import { guid } from "../../semio";
import { Tutorial } from "./store";

export const helloTutorial: Tutorial = {
  id: guid(),
  name: "Hello Semio Tutorial",
  description: "Learn the basics of Semio by creating your first design",
  totalDuration: 300,
  icon: "🎓",
  image: "/tutorials/hello-semio.png",
  concepts: ["hello-semio", "getting-started", "beginner"],
  milestones: [
    {
      id: guid(),
      title: "Welcome to Semio",
      description: "Let's start by navigating to the home screen",
      commandPattern: {
        command: "semio.sketchpad.navigate",
        argsPattern: ["/"],
      },
      focusElement: {
        selector: "[data-navbar-home]",
        highlightMode: "spotlight",
      },
      cursorAnimation: {
        startX: 100,
        startY: 100,
        endX: 50,
        endY: 50,
        duration: 2,
        action: "click",
      },
      canSkip: true,
      order: 0,
    },
    {
      id: guid(),
      title: "Create a New Kit",
      description: "Click the 'New Kit' button to create your first kit",
      commandPattern: {
        command: "semio.sketchpad.createKit",
      },
      focusElement: {
        selector: "[data-action='create-kit']",
        highlightMode: "pulse",
      },
      canSkip: true,
      order: 1,
    },
    {
      id: guid(),
      title: "Open the Kit",
      description: "Now let's open your newly created kit",
      canSkip: true,
      order: 2,
      duration: 5,
    },
    {
      id: guid(),
      title: "Create a Type",
      description: "Add a new type to your kit",
      commandPattern: {
        command: "semio.kitApp.createType",
      },
      canSkip: true,
      order: 3,
    },
    {
      id: guid(),
      title: "Tutorial Complete!",
      description: "Congratulations! You've completed your first Semio tutorial.",
      canSkip: false,
      order: 4,
      duration: 3,
    },
  ],
};
