// #region Header

// TutorialOverlay.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, useEffect, useState } from "react";
import { useCurrentMilestone, useIsTutorialActive, usePlaybackTime } from "./store";

export const TutorialOverlay: FC = () => {
  const isTutorialActive = useIsTutorialActive();
  const currentMilestone = useCurrentMilestone();
  const playbackTime = usePlaybackTime();
  const [cursorPosition, setCursorPosition] = useState<{ x: number; y: number } | null>(null);

  useEffect(() => {
    if (!currentMilestone?.cursorAnimation) {
      setCursorPosition(null);
      return;
    }
    const animation = currentMilestone.cursorAnimation;
    const progress = Math.min(playbackTime / animation.duration, 1);
    const x = animation.startX + (animation.endX - animation.startX) * progress;
    const y = animation.startY + (animation.endY - animation.startY) * progress;
    setCursorPosition({ x, y });
  }, [currentMilestone, playbackTime]);

  if (!isTutorialActive || !currentMilestone) return null;

  return (
    <>
      {currentMilestone.focusElement && <FocusOverlay focusElement={currentMilestone.focusElement} />}
      {cursorPosition && <AnimatedCursor position={cursorPosition} action={currentMilestone.cursorAnimation?.action} />}
      <MilestoneTooltip milestone={currentMilestone} />
    </>
  );
};

interface FocusOverlayProps {
  focusElement: {
    selector: string;
    highlightMode: "dim" | "spotlight" | "pulse";
  };
}

const FocusOverlay: FC<FocusOverlayProps> = ({ focusElement }) => {
  const [rect, setRect] = useState<DOMRect | null>(null);

  useEffect(() => {
    const element = document.querySelector(focusElement.selector);
    if (!element) return;
    const updateRect = () => {
      setRect(element.getBoundingClientRect());
    };
    updateRect();
    const observer = new ResizeObserver(updateRect);
    observer.observe(element);
    window.addEventListener("resize", updateRect);
    return () => {
      observer.disconnect();
      window.removeEventListener("resize", updateRect);
    };
  }, [focusElement.selector]);

  if (!rect) return null;

  return (
    <>
      {focusElement.highlightMode === "dim" && (
        <div className="fixed inset-0 z-[9999] pointer-events-none">
          <svg width="100%" height="100%">
            <defs>
              <mask id="tutorial-mask">
                <rect width="100%" height="100%" fill="white" />
                <rect x={rect.x} y={rect.y} width={rect.width} height={rect.height} fill="black" rx="4" />
              </mask>
            </defs>
            <rect width="100%" height="100%" fill="rgb(0 0 0 / 0.7)" mask="url(#tutorial-mask)" />
          </svg>
          <div className="absolute border-2 border-primary rounded" style={{ left: rect.x - 4, top: rect.y - 4, width: rect.width + 8, height: rect.height + 8 }} />
        </div>
      )}
      {focusElement.highlightMode === "spotlight" && (
        <div className="fixed inset-0 z-[9999] pointer-events-none">
          <div
            className="absolute rounded shadow-[0_0_0_9999px_rgba(0,0,0,0.7)] border-2 border-primary"
            style={{
              left: rect.x - 4,
              top: rect.y - 4,
              width: rect.width + 8,
              height: rect.height + 8,
            }}
          />
        </div>
      )}
      {focusElement.highlightMode === "pulse" && (
        <div className="fixed inset-0 z-[9999] pointer-events-none">
          <div
            className="absolute rounded border-2 border-primary animate-pulse"
            style={{
              left: rect.x - 4,
              top: rect.y - 4,
              width: rect.width + 8,
              height: rect.height + 8,
            }}
          />
        </div>
      )}
    </>
  );
};

interface AnimatedCursorProps {
  position: { x: number; y: number };
  action?: "click" | "drag" | "hover";
}

const AnimatedCursor: FC<AnimatedCursorProps> = ({ position, action }) => {
  return (
    <div className="fixed z-[10000] pointer-events-none transition-all duration-100" style={{ left: position.x, top: position.y }}>
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
        <path d="M3 3L10.07 19.97L12.58 12.58L19.97 10.07L3 3Z" fill="rgb(var(--primary))" stroke="rgb(var(--primary-foreground))" strokeWidth="1" />
      </svg>
      {action === "click" && (
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
          <div className="w-8 h-small rounded-full bg-primary/30 animate-ping" />
        </div>
      )}
    </div>
  );
};

interface MilestoneTooltipProps {
  milestone: {
    title: string;
    description?: string;
  };
}

const MilestoneTooltip: FC<MilestoneTooltipProps> = ({ milestone }) => {
  return (
    <div className="fixed top-small left-1/2 -translate-x-1/2 z-[10001] bg-panel border rounded-lg shadow-lg p-small max-w-md pointer-events-none">
      <h3 className="font-medium text-sm mb-1">{milestone.title}</h3>
      {milestone.description && <p className="text-xs text-muted">{milestone.description}</p>}
    </div>
  );
};
