// #region Header

// Card.tsx

// 2025 Ueli Saluz

// #endregion

import { LucideIcon } from "lucide-react";
import { FC, ReactNode } from "react";

export interface CardProps {
  title: string;
  icon?: string | LucideIcon;
  children: ReactNode;
  className?: string;
}

export const Card: FC<CardProps> = ({ title, icon, children, className = "" }) => {
  const IconComponent = typeof icon === "string" ? null : icon;
  return (
    <div className={`border border-border rounded p-4 bg-panel hover:bg-panel-hover transition-colors ${className}`}>
      <div className="flex items-start gap-3 mb-2">
        {IconComponent && <IconComponent className="w-5 h-5 flex-shrink-0 mt-0.5" />}
        {typeof icon === "string" && <span className="text-xl flex-shrink-0">{icon}</span>}
        <h3 className="font-semibold text-base">{title}</h3>
      </div>
      <div className="text-sm text-muted-foreground">{children}</div>
    </div>
  );
};

export interface CardGridProps {
  stagger?: boolean;
  children: ReactNode;
  className?: string;
}

export const CardGrid: FC<CardGridProps> = ({ stagger = false, children, className = "" }) => {
  return <div className={`grid grid-cols-1 md:grid-cols-2 gap-4 my-6 ${className}`}>{children}</div>;
};
