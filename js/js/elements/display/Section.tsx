// #region Header

// Section.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, ReactNode } from "react";

export interface SectionProps {
  id?: string;
  title?: string;
  children: ReactNode;
  className?: string;
}

const Section: FC<SectionProps> = ({ id, title, children, className = "" }) => {
  return (
    <section id={id} className={`mb-8 ${className}`}>
      {title && (
        <h2 className="text-2xl font-semibold mb-4" id={id}>
          {title}
        </h2>
      )}
      <div>{children}</div>
    </section>
  );
};

export default Section;
