import { FC, useEffect, useState } from "react";
import { useIsFullscreen } from "../../../store";

const Footer: FC = ({}) => {
  const isFullscreen = useIsFullscreen();
  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    if (!isFullscreen) {
      setIsVisible(true);
      return;
    }

    const handleMouseMove = (e: MouseEvent) => {
      // Show footer when mouse is in the bottom 50px of the screen
      setIsVisible(e.clientY > window.innerHeight - 50);
    };

    window.addEventListener("mousemove", handleMouseMove);
    return () => window.removeEventListener("mousemove", handleMouseMove);
  }, [isFullscreen]);

  return (
    <footer className={`flex flex-col gap-4 transition-transform duration-200 ${isFullscreen && !isVisible ? "translate-y-full" : "translate-y-0"}`}>
      <div className="flex flex-col gap-4">
        <h1>Footer</h1>
      </div>
    </footer>
  );
};

export default Footer;
