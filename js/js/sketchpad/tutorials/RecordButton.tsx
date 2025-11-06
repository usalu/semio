// #region Header

// RecordButton.tsx

// Dev mode record button for footer

// #endregion

import { RecordIcon } from "@semio/assets";
import { FC, useEffect } from "react";
import { useAddFooterItem, useMode, useRemoveFooterItem } from "../App";
import { Button } from "../elements";
import { Mode } from "../sketchpad";
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
          className={isRecording ? "text-red-500 h-small w-small p-0" : "h-small w-small p-0"}
        >
          <RecordIcon className={isRecording ? "fill-current size-tiny" : "size-tiny"} />
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
