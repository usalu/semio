
import React from 'react'
import ReactDOM from 'react-dom/client'

import Diagram from '@semio/core/components/ui/Diagram'

function App() {
    return (
        <Diagram />
    );
}

export default App;

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
)