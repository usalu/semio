import { renderToStaticMarkup } from "react-dom/server";
import { BasicChatPanel } from "/home/user/semio/ui/react/index.tsx";
const markup = renderToStaticMarkup(
	<BasicChatPanel id="hub.activity" title="Session activity" messages={[{ id: "m1", author: "Ada", body: "added a piece", color: "#e6194b", timestamp: 0 }, { id: "m2", author: "Claude", body: "created a design", badge: "agent" }]} />,
);
console.log("[DEBUG] author Ada", markup.includes('data-chat-author="Ada"'), "color", markup.includes("background-color:#e6194b"), "no draft", !markup.includes("basic-chat-draft"));
const empty = renderToStaticMarkup(<BasicChatPanel id="x" title="Chat" messages={[]} emptyText="No activity yet" onSend={() => undefined} />);
console.log("[DEBUG] empty", empty.includes("No activity yet"), "draft", empty.includes("basic-chat-draft"));
