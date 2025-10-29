// #region Header

// Chat.tsx

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

import { FC, useState } from "react";
import { useTranslation } from "react-i18next";
import { Textarea } from "../../elements/input/Textarea";
import Panel from "../Panel";
import { ResizablePanelProps } from "../Sketchpad";
import { useIsMobile, useTooltip } from "../store";

interface ChatProps extends ResizablePanelProps {}

const Chat: FC<ChatProps> = ({ visible, onWidthChange, width }) => {
  const { t } = useTranslation();
  const tooltip = useTooltip();
  const isMobile = useIsMobile();
  const [input, setInput] = useState("");

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      // TODO: Handle send message
      setInput("");
    }
  };

  return (
    <Panel
      panelId="chat"
      visible={visible}
      onWidthChange={onWidthChange}
      width={width}
      resizeSide="left"
      footer={
        <div className={`${isMobile ? "p-2" : "p-1"} border-t`}>
          <Textarea value={input} onChange={(e) => setInput(e.target.value)} onKeyDown={handleKeyDown} placeholder={t("semio.sketchpad.panel.chat.placeholder")} i18n="semio.sketchpad.panel.chat.input" />
        </div>
      }
    />
  );
};

export default Chat;
