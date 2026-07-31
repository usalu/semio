import { StrictMode, useEffect } from "react";
import { createRoot } from "react-dom/client";
import { Icon, LevelProvider, Panel, singleTreeLeaf } from "@semio-tech/ui-react";
import "./probe.css";

//#region 🧪️Probe
const EmptyIcon = () => null;

function Probe() {
  useEffect(() => {
    requestAnimationFrame(() => {
      const host = document.querySelector<HTMLElement>('[data-testid="probe-host"]');
      const panel = document.querySelector<HTMLElement>('[data-slot="panel"]');
      const tree = document.querySelector<HTMLElement>('[data-testid="probe-tree"]');
      const rows = Array.from(document.querySelectorAll<HTMLElement>('[data-slot="tree-section-row"], [data-slot="tree-item-row"]'));
      const widths = {
        host: host?.getBoundingClientRect().width,
        panel: panel?.getBoundingClientRect().width,
        tree: tree?.getBoundingClientRect().width,
        rows: rows.map((row) => row.getBoundingClientRect().width),
        panelRightGap: host && panel ? host.getBoundingClientRect().right - panel.getBoundingClientRect().right : undefined,
        rowsInline: rows.map((row) => ({
          id: row.id,
          direction: getComputedStyle(row).direction,
          left: row.querySelector<HTMLElement>('[data-slot="tree-label"]')?.getBoundingClientRect().left,
          width: row.querySelector<HTMLElement>('[data-slot="tree-row-content"]')?.getBoundingClientRect().width,
        })),
      };
      console.log(`[DEBUG] tree-width-probe ${JSON.stringify(widths)}`);
    });
  }, []);

  const tab = singleTreeLeaf({
    id: "probe.tree",
    icon: EmptyIcon,
    name: "History",
    tree: {
      sections: [
        {
          id: "framework.history.commands",
          label: "Commands",
          defaultOpen: true,
          items: [
            { id: "framework.history.entry.1", label: "Set Active Example", icon: <Icon icon="pencil" size={12} /> },
            { id: "framework.history.entry.2", label: "Resize Window", icon: <Icon icon="monitor" size={12} /> },
            { id: "framework.history.entry.3", label: "Select Brush Mesh x2", icon: <Icon icon="eye" size={12} /> },
            { id: "framework.history.entry.4", label: "Toggle Panel", icon: <Icon icon="monitor" size={12} /> },
            { id: "framework.history.entry.5", label: "Switch Panel Tab", icon: <Icon icon="monitor" size={12} /> },
          ],
        },
      ],
      className: "w-full min-w-0",
    },
  });

  return (
    <StrictMode>
      <LevelProvider level="base">
        <div data-testid="probe-host" className="relative h-[400px] w-[800px] overflow-hidden bg-background">
          <Panel anchor="bottom-right" visible size={500} tabs={[tab]} activeTabPath={[tab.id]} />
        </div>
      </LevelProvider>
    </StrictMode>
  );
}

createRoot(document.getElementById("root")!).render(<Probe />);
//#endregion 🧪️Probe
