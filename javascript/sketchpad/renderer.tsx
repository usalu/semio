
import React from 'react'
import ReactDOM from 'react-dom/client'

// import Diagram from '@semio/core/components/ui/Diagram'

function App() {
    return (
        <div className="h-screen w-screen flex justify-center items-center bg-pink-600">
            <h1>semio</h1>
        </div>
        // <Diagram />
    );
}

export default App;

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
)