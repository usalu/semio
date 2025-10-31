// #region Header

// RecordButton.tsx

// Dev mode record button for footer

// #endregion

import { Circle } from "lucide-react";
import { FC, useEffect } from "react";
import { Button } from "../../elements/input/Button";
import { useAddFooterItem, useRemoveFooterItem } from "../Footer";
import { Mode, useMode } from "../store";
import { useIsRecording, useTutorialStore } from "./store";

export const RecordButton: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const mode = useMode();
  const isRecording = useIsRecording();
  const store = useTutorialStore();

  useEffect(() => {
    if (mode !== Mode.DEV) {
      removeFooterItem("record-button");
      return;
    }

    addFooterItem({
      id: "record-button",
      content: (
        <Button
          id="footer-record-button"
          variant="ghost"
          onClick={() => {
            if (isRecording) {
              store.stopRecording();
            } else {
              store.startRecording("New Recording", undefined);
            }
          }}
          className={isRecording ? "text-red-500 h-5 w-8 p-0" : "h-5 w-8 p-0"}
        >
          <Circle className={isRecording ? "fill-current h-3 w-3" : "h-3 w-3"} />
        </Button>
      ),
      order: 0,
    });

    return () => {
      removeFooterItem("record-button");
    };
  }, [mode, isRecording, store, addFooterItem, removeFooterItem]);

  return null;
};
