// #region Header

// Steps.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, ReactNode } from "react";

export interface StepsProps {
  children: ReactNode;
  className?: string;
}

export const Steps: FC<StepsProps> = ({ children, className = "" }) => {
  return <div className={`steps-container space-y-6 my-6 ${className}`}>{children}</div>;
};
