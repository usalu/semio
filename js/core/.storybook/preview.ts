import type { Preview } from '@storybook/react'

import '../globals.css';

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
    darkMode: {
      current: 'auto',
      dark: { appBg: '-var(--color-dark)' },
      light: { appBg: '-var(--color-dark)' }
    }
  },

  tags: ['autodocs']
};

export default preview;