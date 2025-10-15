import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

import "./globals.css";

import { Sketchpad } from "@semio/js";

declare global {
  interface Window {
    windowControls: {
      minimize(): Promise<any>;
      maximize(): Promise<any>;
      close(): Promise<any>;
    };
    os: {
      getUserId(): Promise<string>;
    };
  }
}

const invokeWindowControl = (action: "minimize" | "maximize" | "close") => {
  if (window.windowControls) {
    return window.windowControls[action]();
  }
  console.warn(`Window controls not available for action: ${action}`);
  return Promise.resolve();
};

const windowEvents = {
  minimize: () => invokeWindowControl("minimize"),
  maximize: () => invokeWindowControl("maximize"),
  close: () => invokeWindowControl("close"),
};

const os = {
  getUserId: async () => await window.os.getUserId(),
};

function App() {
  const [userId, setUserId] = useState<string>("");

  useEffect(() => {
    async function fetchUserId() {
      try {
        const id = await os.getUserId();
        setUserId(id);
      } catch (error) {
        console.error("Failed to get user ID:", error);
        setUserId("anonymous-user");
      }
    }

    fetchUserId();
  }, []);

  return <div className="h-screen w-screen">{userId ? <Sketchpad onWindowEvents={windowEvents} userId={userId} /> : <div className="flex h-full w-full items-center justify-center">Loading user data...</div>}</div>;
}

export default App;

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
