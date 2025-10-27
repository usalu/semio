// #region Header

// Aside.tsx

// 2025 Ueli Saluz

// #endregion

import { AlertCircle, Info, Lightbulb, TriangleAlert } from "lucide-react";
import { FC, ReactNode } from "react";

export interface AsideProps {
  type?: "note" | "tip" | "caution" | "danger";
  title?: string;
  children: ReactNode;
}

const iconMap = {
  note: Info,
  tip: Lightbulb,
  caution: TriangleAlert,
  danger: AlertCircle,
};

const colorMap = {
  note: "border-info-border bg-info-bg text-info-foreground",
  tip: "border-success-border bg-success-bg text-success-foreground",
  caution: "border-warning-border bg-warning-bg text-warning-foreground",
  danger: "border-destructive-border bg-destructive-bg text-destructive-foreground",
};

export const Aside: FC<AsideProps> = ({ type = "note", title, children }) => {
  const Icon = iconMap[type];
  const colorClass = colorMap[type];

  return (
    <aside className={`my-4 p-4 border ${colorClass}`}>
      <div className="flex items-start gap-2">
        <Icon className="w-5 h-5 mt-0.5 flex-shrink-0" />
        <div className="flex-1">
          {title && <div className="font-semibold mb-1">{title}</div>}
          <div>{children}</div>
        </div>
      </div>
    </aside>
  );
};
